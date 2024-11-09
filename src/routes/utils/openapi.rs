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

use utoipa::openapi::{
    security::{ApiKey, ApiKeyValue, SecurityScheme},
    LicenseBuilder, OpenApi,
};

use crate::models::file::FileType;

struct AxumOidcSecurity;
impl utoipa::Modify for AxumOidcSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.info.license = Some(
            LicenseBuilder::new()
                .name("CLOSED")
                .identifier(Some("CLOSED"))
                .build(),
        );
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "axum-oidc",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("id"))),
            )
        }
    }
}

#[derive(utoipa::OpenApi)]
#[openapi(
    modifiers(&AxumOidcSecurity),
    components(
        schemas(FileType)
    ),
)]
pub struct ApiDoc;

pub const USER_TAG: &str = "user";
pub const LOCATION_TAG: &str = "location";
pub const PRODUCT_TAG: &str = "product";
pub const REFILL_TAG: &str = "refill";
pub const MISC_TAG: &str = "misc";

/// Configures the OpenAPI documentation routes.
///
/// This function sets up the routes for serving OpenAPI documentation.
/// It configures the Swagger UI to be available at: `/swagger-ui`
///
/// # Behavior
/// - **Debug Mode**: If the application is running in debug mode the function will return a router with Swagger UI and OpenAPI schema endpoints enabled.
/// - **Release Mode**: In release mode, it returns an empty router with no OpenAPI documentation routes.
pub fn openapi(path: &str, api: OpenApi) -> axum::Router<crate::state::AppState> {
    // Enable openapi documentation only in debug (not in release)
    if cfg!(debug_assertions) {
        axum::Router::new().merge(
            utoipa_swagger_ui::SwaggerUi::new(format!("{path}/swagger-ui"))
                .url(format!("{path}/api-docs/openapi.json"), api),
        )
    } else {
        axum::Router::new()
    }
}
