//! This module defines the API endpoint to delete a location by its ID.
//!
//! Only an admin can delete a location.

use crate::routes::utils::openapi::LOCATION_TAG;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::error::AppError;
use extractor::profile::admin::Admin;
use service::Connection;

/// Deletes a location by its database ID.
///
/// The location is not fully removed but marked as disabled in the database.
/// Only an admin can perform this action.
///
/// - **Path Parameters:**
///   - `id`: The unique ID of the location in the database.
///
/// - **Responses:**
///   - `500`: Internal error, likely related to the database.
///   - `400`: The request format is invalid.
///   - `200`: The location has been successfully disabled.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = LOCATION_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "Location database id to delete location for"),
    ),
    responses(
        (status = 500, description = "An internal error occured, probably databse related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The location is disabled")
    ),
    security(
        ("axum-oidc" = [""])
    )
)]
pub async fn delete_location(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_location_by_id(&conn, id).await?;

    match result {
        Some(location) => {
            service::Mutation::delete_location(&conn, id).await?;

            tracing::info!(
                "{admin} just deleted the location {} \"{}\" - {:?}",
                location.name,
                id,
                location
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The location with id: {id} doesn't exist"
        ))),
    }
}
