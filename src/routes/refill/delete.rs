//! This module defines the API endpoint to delete a refill by its ID.
//!
//! Only an admin can delete a refill.

use crate::{error::AppError, models::profile::admin::Admin, routes::utils::openapi::REFILL_TAG};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use service::Connection;

/// Deletes a refill by its database ID.
///
/// The refill is not fully removed but marked as disabled in the database.
/// Only an admin can perform this action.
///
/// - **Path Parameters:**
///   - `id`: The unique ID of the refill in the database.
///
/// - **Responses:**
///   - `500`: Internal error, likely related to the database.
///   - `400`: The request format is invalid.
///   - `200`: The refill has been successfully disabled.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = REFILL_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "refill database id to delete refill for"),
    ),
    responses(
        (status = 500, description = "An internal error occured, probably databse related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The refill is disabled")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn delete_refill(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_refill_by_id(&conn, id).await?;

    match result {
        Some(refill) => {
            service::Mutation::delete_refill(&conn, id).await?;

            tracing::info!("{admin} just deleted the refill \"{}\" - {:?}", id, refill);

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The refill with id: {id} doesn't exist"
        ))),
    }
}
