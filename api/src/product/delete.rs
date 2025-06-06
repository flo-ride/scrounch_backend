//! This module defines the API endpoint to delete a product by its ID.
//!
//! Only an admin can delete a product.

use crate::utils::openapi::PRODUCT_TAG;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::error::AppError;
use extractor::profile::admin::Admin;
use service::Connection;

/// Deletes a product by its database ID.
///
/// The product is not fully removed but marked as disabled in the database.
/// Only an admin can perform this action.
///
/// - **Path Parameters:**
///   - `id`: The unique ID of the product in the database.
///
/// - **Responses:**
///   - `500`: Internal error, likely related to the database.
///   - `400`: The request format is invalid.
///   - `200`: The product has been successfully disabled.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = PRODUCT_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "Product database id to delete product for"),
    ),
    responses(
        (status = 500, description = "An internal error occured, probably databse related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The product is disabled")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn delete_product(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_product_by_id(&conn, id).await?;

    match result {
        Some(product) => {
            service::Mutation::delete_product(&conn, id).await?;

            log::info!(
                "{admin} just deleted the product {} \"{}\" - {:?}",
                product.name,
                id,
                product
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The product with id: {id} doesn't exist"
        ))),
    }
}
