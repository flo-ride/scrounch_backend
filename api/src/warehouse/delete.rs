//! This module defines the API endpoint to delete a warehouse by its ID.
//!
//! Only an admin can delete a warehouse.

use crate::utils::openapi::WAREHOUSE_TAG;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::error::AppError;
use extractor::profile::admin::Admin;
use service::Connection;

/// Deletes a warehouse by its database ID.
///
/// The warehouse is not fully removed but marked as disabled in the database.
/// Only an admin can perform this action.
///
/// - **Path Parameters:**
///   - `id`: The unique ID of the warehouse in the database.
///
/// - **Responses:**
///   - `500`: Internal error, likely related to the database.
///   - `400`: The request format is invalid.
///   - `200`: The warehouse has been successfully disabled.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = WAREHOUSE_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "Warehouse database id to delete warehouse for"),
    ),
    responses(
        (status = 500, description = "An internal error occured, probably databse related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The warehouse is disabled")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn delete_warehouse(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_warehouse_by_id(&conn, id).await?;

    match result {
        Some(warehouse) => {
            service::Mutation::delete_warehouse(&conn, id).await?;

            log::info!(
                "{admin} just deleted the warehouse  \"{}\" - {:?}",
                id,
                warehouse
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The warehouse with id: {id} doesn't exist"
        ))),
    }
}
