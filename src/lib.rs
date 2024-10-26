#![warn(missing_docs)]
//! scrounch_backend - Backend for a sales application
//!
//! This library serves as the core of the `scrounch_backend` crate, providing
//! all backend functionality for managing beverage sales. It integrates the Axum
//! framework for handling HTTP requests and response routing.

mod cli;
mod error;
mod extractors;
mod handlers;
mod models;
mod oidc;
mod routes;
mod state;

use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    http::Method,
    routing::{get, post},
};
use axum_oidc::EmptyAdditionalClaims;
pub use cli::Arguments;
use migration::MigratorTrait;
use oidc::handle_axum_oidc_middleware_error;

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
    .await
    .expect("Couldn't connect to S3 Bucket");

    migration::Migrator::up(&db_pool, None)
        .await
        .expect("Migration couldn't proceed correctly");

    let mut state = state::AppState {
        arguments: arguments.clone(),
        db_pool,
        #[cfg(feature = "cache")]
        cache_pool: None,
        s3_bucket,
    };

    #[cfg(feature = "cache")]
    if let Some(cache_url) = arguments.cache_url.clone() {
        use fred::interfaces::ClientLike;

        let config = fred::types::RedisConfig::from_url(&cache_url)
            .expect("Cache URL is not correctly formatted");
        let mut builder = fred::types::Builder::from_config(config);
        builder
            .set_policy(fred::types::ReconnectPolicy::new_exponential(
                0, 100, 10_000, 2,
            ))
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

    if path != "/" {
        tracing::info!("Server with use path: {path}");
    }

    #[cfg(feature = "cache")]
    if let Some(pool) = state.cache_pool.clone() {
        return axum::Router::new()
            .merge(auth_required_routes(&path))
            .layer(login_service)
            .merge(auth_optional_routes(&path))
            .layer(auth_service)
            .layer(oidc::cache_session_layer(pool))
            .merge(routes::utils::openapi::openapi(&path))
            .layer(cors_layer)
            .with_state(state);
    }

    axum::Router::new()
        .merge(auth_required_routes(&path))
        .layer(login_service)
        .merge(auth_optional_routes(&path))
        .layer(auth_service)
        .layer(oidc::memory_session_layer())
        .merge(routes::utils::openapi::openapi(&path))
        .layer(cors_layer)
        .with_state(state)
}

/// Defines routes that require user authentication.
///
/// This function creates an `axum::Router` specifically for routes that are
/// protected by authentication. These routes require the user to be logged in
/// and authenticated via OpenID Connect (OIDC) to access, otherwise it redirect them to the OIDC
/// login page.
fn auth_required_routes(path: &str) -> axum::Router<state::AppState> {
    axum::Router::new().nest(
        path,
        axum::Router::new()
            .route("/login", get(routes::utils::login::get_login))
            .route("/logout", get(routes::utils::logout::get_logout)),
    )
}

/// Defines routes that do not require user authentication.
///
/// This function creates an `axum::Router` for routes that can be accessed
/// without user authentication. These routes are publicly accessible and do not
/// require OpenID Connect (OIDC) login.
fn auth_optional_routes(path: &str) -> axum::Router<state::AppState> {
    axum::Router::new().nest(
        path,
        axum::Router::new()
            .route("/me", get(routes::user::me::get_me))
            .route("/upload", post(routes::utils::upload::post_upload_files))
            .route(
                "/download/:filename",
                get(routes::utils::download::download_file),
            )
            .route("/status", get(routes::utils::status::get_status))
            .route("/sma", post(routes::utils::sma::post_update_from_sma))
            .nest("/product", routes::product::router()),
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
    bucket_name: String,
    region: String,
    endpoint: String,
    access_key: String,
    secret_key: String,
) -> Result<s3::Bucket, s3::error::S3Error> {
    let region = s3::Region::Custom { region, endpoint };
    let credentials =
        s3::creds::Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)?;
    let bucket =
        s3::Bucket::new(&bucket_name, region.clone(), credentials.clone())?.with_path_style();

    match bucket.exists().await? {
        true => Ok(*bucket),
        false => {
            s3::Bucket::create_with_path_style(
                &bucket_name,
                region,
                credentials,
                s3::BucketConfiguration::default(),
            )
            .await?;
            Ok(*bucket)
        }
    }
}
