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

use entity::request::product::{EditProductRequest, NewProductRequest};
use utoipa::OpenApi;

use crate::models::file::FileType;
use crate::models::response::product::*;
use crate::models::response::user::{UserListResponse, UserResponse};
use crate::routes::product::delete::__path_delete_product;
use crate::routes::product::edit::__path_edit_product;
use crate::routes::product::get::{__path_get_all_products, __path_get_product};
use crate::routes::product::new::__path_post_new_product;
use crate::routes::user::edit::__path_edit_user;
use crate::routes::user::get::{__path_get_all_users, __path_get_user};
use crate::routes::user::me::__path_get_me;
use crate::routes::utils::download::__path_download_file;
use crate::routes::utils::login::__path_get_login;
use crate::routes::utils::logout::__path_get_logout;
use crate::routes::utils::status::__path_get_status;
use crate::routes::utils::upload::{FileSchema, __path_post_upload_files};
use entity::request::user::EditUserRequest;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_status,
        get_login,
        get_logout,
        get_me,
        post_upload_files,
        download_file,
        get_product,
        get_all_products,
        post_new_product,
        edit_product,
        delete_product,
        get_user,
        get_all_users,
        edit_user,
    ),
    components(
        schemas(EditUserRequest),
        schemas(UserListResponse),
        schemas(UserResponse),
        schemas(FileType),
        schemas(FileSchema),
        schemas(NewProductRequest),
        schemas(EditProductRequest),
        schemas(ProductResponse),
        schemas(ProductListResponse)
    )
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
pub fn openapi(path: &str) -> axum::Router<crate::state::AppState> {
    // Enable openapi documentation only in debug (not in release)
    if cfg!(debug_assertions) {
        let path = match path {
            "/" => "",
            _ => path,
        };

        axum::Router::new().merge(
            utoipa_swagger_ui::SwaggerUi::new(format!("{path}/swagger-ui"))
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    } else {
        axum::Router::new()
    }
}
