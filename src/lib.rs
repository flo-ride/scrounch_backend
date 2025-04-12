#![warn(missing_docs)]
//! scrounch_backend - Backend for a sales application
//!
//! This library serves as the core of the `scrounch_backend` crate, providing
//! all backend functionality for managing beverage sales. It integrates the Axum
//! framework for handling HTTP requests and response routing.

mod cli;
mod oidc;
mod state;

use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::Method};
use axum_oidc::EmptyAdditionalClaims;
pub use cli::Arguments;
use migration::MigratorTrait;
use oidc::handle_axum_oidc_middleware_error;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Creates and configures the Axum application.
///
/// This function sets up the Axum app, defines the routes, middleware,
/// and any other necessary configuration for handling HTTP requests.
/// It wires up the backend services such as authentication, database connections,
/// and any other business logic needed to manage the beverage sales system.
pub async fn app(arguments: Arguments) -> axum::Router {
    let db_pool = get_database_conn(&arguments.database_url, None)
        .await
        .expect("Couldn't connect to the database");

    let s3_bucket = get_bucket_conn(
        arguments.aws_s3_bucket.clone(),
        arguments.aws_region.clone(),
        arguments.aws_endpoint_url.clone(),
        arguments.aws_access_key_id.clone(),
        arguments.aws_secret_access_key.clone(),
    )
    .await;

    migration::Migrator::up(&db_pool, None)
        .await
        .expect("Migration couldn't proceed correctly");

    let mut state = state::AppState {
        arguments: Arguments::default(),
        db_pool,
        #[cfg(feature = "cache")]
        cache_pool: None,
        s3_storage: s3_bucket,
    };

    state.arguments = arguments.clone();

    #[cfg(feature = "cache")]
    if let Some(cache_url) = arguments.cache_url.clone() {
        use fred::interfaces::ClientLike;

        let config = fred::types::RedisConfig::from_url(&cache_url)
            .expect("Cache URL is not correctly formatted");
        let mut builder = fred::types::Builder::from_config(config);
        builder
            .set_policy(fred::types::ReconnectPolicy::new_constant(4, 1500))
            .with_config(|c| {
                c.fail_fast = false;
                c.tracing.enabled = true;
            });
        let cache_pool = builder
            .build_pool(8)
            .expect("Could not connect to Cache Pool");
        cache_pool.init().await.expect("Failed to connect to Cache");
        tracing::info!("Cache is connected");

        state.cache_pool = Some(cache_pool)
    }

    let login_service = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_axum_oidc_middleware_error))
        .layer(axum_oidc::OidcLoginLayer::<EmptyAdditionalClaims>::new());

    let oidc_client = oidc::get_oidc_client(&arguments)
        .await
        .expect("Can't create OIDC client");

    let auth_service = tower::ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_axum_oidc_middleware_error))
        .layer(oidc_client);

    let origins = [
        arguments.frontend_url.parse().unwrap(),
        arguments.backend_url.parse().unwrap(),
    ];

    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
        ])
        .allow_credentials(true)
        .allow_origin(origins);

    let path = url::Url::parse(&arguments.backend_url)
        .expect("Cannot parse backend_url")
        .path()
        .to_string();

    let path = match path.as_str() {
        "/" => "".to_string(),
        _ => {
            tracing::info!("Server use path: {path}");
            path
        }
    };

    let (router, api) = OpenApiRouter::with_openapi(api::utils::openapi::ApiDoc::openapi())
        .merge(auth_required_routes(&path))
        .layer(login_service)
        .merge(auth_optional_routes(&path))
        .layer(axum::extract::DefaultBodyLimit::max(64000000))
        .split_for_parts();

    let cookie_duration = arguments.openid_token_duration;

    #[cfg(feature = "cache")]
    if let Some(pool) = state.cache_pool.clone() {
        return router
            .layer(auth_service)
            .layer(oidc::cache_session_layer(pool, cookie_duration))
            .merge(api::utils::openapi::openapi(&path, api))
            .layer(cors_layer)
            .with_state(state);
    }

    router
        .layer(auth_service)
        .layer(oidc::memory_session_layer(cookie_duration))
        .merge(api::utils::openapi::openapi(&path, api))
        .layer(cors_layer)
        .with_state(state)
}

/// Defines routes that require user authentication.
///
/// This function creates an `axum::Router` specifically for routes that are
/// protected by authentication. These routes require the user to be logged in
/// and authenticated via OpenID Connect (OIDC) to access, otherwise it redirect them to the OIDC
/// login page.
fn auth_required_routes(path: &str) -> OpenApiRouter<state::AppState> {
    OpenApiRouter::new().nest(
        path,
        OpenApiRouter::new()
            .routes(routes!(api::utils::login::get_login))
            .routes(routes!(api::utils::logout::get_logout)),
    )
}

/// Defines routes that do not require user authentication.
///
/// This function creates an `axum::Router` for routes that can be accessed
/// without user authentication. These routes are publicly accessible and do not
/// require OpenID Connect (OIDC) login.
fn auth_optional_routes(path: &str) -> OpenApiRouter<state::AppState> {
    OpenApiRouter::new().nest(
        path,
        OpenApiRouter::new()
            .routes(routes!(api::user::me::get_me))
            .routes(routes!(api::utils::upload::post_upload_files))
            .routes(routes!(api::utils::download::download_file))
            .routes(routes!(api::utils::status::get_status))
            .routes(routes!(api::utils::sma::post_update_from_sma))
            .nest(
                "/product",
                OpenApiRouter::new()
                    .routes(routes!(api::product::get::get_product))
                    .routes(routes!(api::product::get::get_all_products))
                    .routes(routes!(api::product::new::post_new_product))
                    .routes(routes!(api::product::edit::edit_product))
                    .routes(routes!(api::product::delete::delete_product)),
            )
            .nest(
                "/user",
                OpenApiRouter::new()
                    .routes(routes!(api::user::get::get_user))
                    .routes(routes!(api::user::get::get_all_users))
                    .routes(routes!(api::user::edit::edit_user)),
            )
            .nest(
                "/location",
                OpenApiRouter::new()
                    .routes(routes!(api::location::get::get_location))
                    .routes(routes!(api::location::get::get_all_locations))
                    .routes(routes!(api::location::new::post_new_location))
                    .routes(routes!(api::location::edit::edit_location))
                    .routes(routes!(api::location::delete::delete_location)),
            )
            .nest(
                "/refill",
                OpenApiRouter::new()
                    .routes(routes!(api::refill::get::get_refill))
                    .routes(routes!(api::refill::get::get_all_refills))
                    .routes(routes!(api::refill::new::post_new_refill))
                    .routes(routes!(api::refill::edit::edit_refill))
                    .routes(routes!(api::refill::delete::delete_refill)),
            )
            .nest(
                "/recipe",
                OpenApiRouter::new()
                    .routes(routes!(api::recipe::get::get_recipe))
                    .routes(routes!(api::recipe::get::get_all_recipes))
                    .routes(routes!(api::recipe::new::post_new_recipe))
                    .routes(routes!(api::recipe::edit::edit_recipe))
                    .routes(routes!(api::recipe::delete::delete_recipe)),
            )
            .nest(
                "/warehouse",
                OpenApiRouter::new()
                    .routes(routes!(api::warehouse::get::get_warehouse))
                    .routes(routes!(api::warehouse::get::get_all_warehouses))
                    .routes(routes!(api::warehouse::new::post_new_warehouse))
                    .routes(routes!(api::warehouse::edit::edit_warehouse))
                    .routes(routes!(api::warehouse::delete::delete_warehouse))
                    .routes(routes!(api::warehouse::new::post_new_warehouse_product))
                    .routes(routes!(api::warehouse::get::get_warehouse_product))
                    .routes(routes!(api::warehouse::get::get_all_warehouse_products)),
            ),
    )
}

async fn get_database_conn(
    url: &str,
    default_schema: Option<String>,
) -> Result<sea_orm::prelude::DatabaseConnection, sea_orm::prelude::DbErr> {
    let mut opt = sea_orm::ConnectOptions::new(url);
    opt.max_connections(50)
        .min_connections(3)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    if let Some(default_schema) = default_schema {
        opt.set_schema_search_path(default_schema);
    }

    sea_orm::Database::connect(opt).await
}

async fn get_bucket_conn(
    bucket: String,
    region_name: String,
    endpoint: String,
    access_key: String,
    secret_key: String,
) -> entity::s3::S3FileStorage {
    let credentials = aws_sdk_s3::config::Credentials::new(
        access_key,
        secret_key,
        None,
        None,
        "loaded-from-custom-env",
    );

    let region = aws_config::Region::new(region_name.clone());

    let s3_config = aws_sdk_s3::config::Builder::new()
        .endpoint_url(endpoint)
        .credentials_provider(credentials)
        .region(region.clone())
        .force_path_style(true)
        .build();
    let client = aws_sdk_s3::Client::from_conf(s3_config);

    let constraint = aws_sdk_s3::types::BucketLocationConstraint::from(region_name.as_str());

    let config = aws_sdk_s3::types::CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    client
        .create_bucket()
        .create_bucket_configuration(config)
        .bucket(&bucket)
        .send()
        .await
        .ok();

    entity::s3::S3FileStorage { bucket, client }
}
