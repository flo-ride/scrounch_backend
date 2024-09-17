#![warn(missing_docs)]
//! scrounch_backend - Backend for a sales application
//!
//! This library serves as the core of the `scrounch_backend` crate, providing
//! all backend functionality for managing beverage sales. It integrates the Axum
//! framework for handling HTTP requests and response routing.

mod cli;
mod oidc;
mod routes;
mod state;

use axum::{error_handling::HandleErrorLayer, http::Method, routing::get};
use axum_oidc::EmptyAdditionalClaims;
pub use cli::Arguments;
use oidc::handle_axum_oidc_middleware_error;
use sea_orm::DatabaseConnection;

/// Creates and configures the Axum application.
///
/// This function sets up the Axum app, defines the routes, middleware,
/// and any other necessary configuration for handling HTTP requests.
/// It wires up the backend services such as authentication, database connections,
/// and any other business logic needed to manage the beverage sales system.
pub async fn app(arguments: Arguments, db_pool: DatabaseConnection) -> axum::Router {
    let state = state::AppState {
        arguments: arguments.clone(),
        db_pool,
    };

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
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true)
        .allow_origin(origins);

    axum::Router::new()
        .merge(auth_required_routes())
        .layer(login_service)
        .merge(auth_optional_routes())
        .layer(auth_service)
        .layer(oidc::memory_session_layer())
        .merge(routes::utils::openapi::openapi())
        .route("/status", get(routes::utils::status::get_status))
        .layer(cors_layer)
        .with_state(state)
}

/// Defines routes that require user authentication.
///
/// This function creates an `axum::Router` specifically for routes that are
/// protected by authentication. These routes require the user to be logged in
/// and authenticated via OpenID Connect (OIDC) to access, otherwise it redirect them to the OIDC
/// login page.
fn auth_required_routes() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route("/login", get(routes::utils::login::get_login))
        .route("/logout", get(routes::utils::logout::get_logout))
}

/// Defines routes that do not require user authentication.
///
/// This function creates an `axum::Router` for routes that can be accessed
/// without user authentication. These routes are publicly accessible and do not
/// require OpenID Connect (OIDC) login.
fn auth_optional_routes() -> axum::Router<state::AppState> {
    axum::Router::new().route("/me", get(routes::user::me::get_me))
}
