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

use service::s3::FileType;
use utoipa::openapi::{
    security::{ApiKey, ApiKeyValue, SecurityScheme},
    LicenseBuilder, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

/// Custom security configuration for OpenAPI in the Axum-OIDC integration.
///
/// This struct implements the `utoipa::Modify` trait to modify the OpenAPI schema
/// to include a license and a security scheme specific to the Axum OIDC configuration.
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

/// OpenAPI documentation for the `scrounch_backend` API.
///
/// The `ApiDoc` struct configures the OpenAPI documentation for the API,
/// including security, component schemas, and various tagged endpoint categories.
#[derive(utoipa::OpenApi)]
#[openapi(
    modifiers(&AxumOidcSecurity),
    components(
        schemas(FileType),
        schemas(entity::models::product::ProductSortEnum),
        schemas(entity::models::refill::RefillSortEnum),
        schemas(entity::models::location::LocationSortEnum),
        schemas(entity::models::user::UserSortEnum),
        schemas(entity::models::recipe::RecipeSortEnum),
        schemas(entity::models::warehouse::WarehouseSortEnum),
        schemas(entity::models::warehouse_product::Warehouse_productSortEnum),
    ),
)]
pub struct ApiDoc;

/// Tag used to categorize API endpoints related to user management and user-specific actions.
pub const USER_TAG: &str = "user";

/// Tag used to categorize API endpoints for managing and accessing location-based data.
pub const LOCATION_TAG: &str = "location";

/// Tag used to categorize API endpoints related to product creation, retrieval, and management.
pub const PRODUCT_TAG: &str = "product";

/// Tag used to categorize API endpoints focused on refills and related operations.
pub const REFILL_TAG: &str = "refill";

/// Tag used to categorize API endpoints focused on recipe and related operations.
pub const RECIPE_TAG: &str = "recipe";

/// Tag used to categorize API endpoints focused on warehouse and related operations.
pub const WAREHOUSE_TAG: &str = "warehouse";

/// Tag used to categorize miscellaneous API endpoints that do not fit into other categories.
pub const MISC_TAG: &str = "misc";

/// Configures the OpenAPI documentation routes.
///
/// This function sets up the routes for serving OpenAPI documentation.
/// It configures the Swagger UI to be available at: `/swagger-ui`
///
/// # Behavior
/// - **Debug Mode**: If the application is running in debug mode the function will return a router with Swagger UI and OpenAPI schema endpoints enabled.
/// - **Release Mode**: In release mode, it returns an empty router with no OpenAPI documentation routes.
pub fn openapi(path: &str, api: OpenApi) -> SwaggerUi {
    utoipa_swagger_ui::SwaggerUi::new(format!("{path}/swagger-ui"))
        .url(format!("{path}/api-docs/openapi.json"), api)
}
