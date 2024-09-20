//! OpenAPI documentation routes.
//!
//! This module defines routes related to serving OpenAPI documentation for the
//! `scrounch_backend` application. It provides endpoints to access the OpenAPI
//! schema and possibly other related documentation, allowing clients to understand
//! and interact with the API.
//!
//! # Features
//! - Serving the OpenAPI schema in JSON format.
//! - Providing documentation for the API endpoints as specified by the OpenAPI standard.

use utoipa::OpenApi;

use crate::models::oidc_user::OidcUser;
use crate::routes::user::me::__path_get_me;
use crate::routes::utils::login::__path_get_login;
use crate::routes::utils::logout::__path_get_logout;
use crate::routes::utils::status::__path_get_status;

#[derive(OpenApi)]
#[openapi(
    paths(get_status, get_login, get_logout, get_me),
    components(schemas(OidcUser))
)]
struct ApiDoc;

/// Configures the OpenAPI documentation routes.
///
/// This function sets up the routes for serving OpenAPI documentation.
/// It configures the Swagger UI to be available at: `/swagger-ui`
///
/// # Behavior
/// - **Debug Mode**: If the application is running in debug mode the function will return a router with Swagger UI and OpenAPI schema endpoints enabled.
/// - **Release Mode**: In release mode, it returns an empty router with no OpenAPI documentation routes.
pub fn openapi() -> axum::Router<crate::state::AppState> {
    // Enable openapi documentation only in debug (not in release)
    if cfg!(debug_assertions) {
        axum::Router::new().merge(
            utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    } else {
        axum::Router::new()
    }
}
