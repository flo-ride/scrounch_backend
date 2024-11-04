//! This module contains the route handler for creating a new refill.
//!
//! The handler will be accessible via a POST request to the `/refill` endpoint.
//! It allows for the creation of new refill entries in the database.
//! Admin privileges are required to access this route.

use crate::error::AppError;
use crate::models::profile::admin::Admin;
use crate::routes::utils::openapi::REFILL_TAG;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::models::refill::ActiveModel;
use entity::request::refill::NewRefillRequest;
use service::Connection;

/// Handler for creating a new refill.
///
/// This function allows an admin to create a new refill by sending a POST request to the `/refill` endpoint.
/// The new refill is validated and stored in the database. The image associated with the refill is checked in S3 storage.
///
/// - **Admin privileges** are required to access this route.
/// - Returns a `201 Created` status upon successful creation along with the refill's ID.
///
/// Path: `/refill`
///
/// - **Request Body:** Expects a `NewRefill` JSON object.
/// - **Responses:**
///     - 500: Internal server error (likely database related).
///     - 400: Bad request (invalid input data).
///     - 201: Successfully created a new refill, returns the new refill's ID as a string.
#[utoipa::path(
    post,
    path = "", 
    tag = REFILL_TAG,
    request_body(content = NewRefillRequest, content_type = "application/json"), 
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 201, description = "Successfully created a new refill, returns the new refill's ID as a string.", body = uuid::Uuid)
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn post_new_refill(
    admin: Admin,
    State(conn): State<Connection>,
    Json(refill): Json<NewRefillRequest>,
) -> Result<impl IntoResponse, AppError> {
    let refill: ActiveModel = refill.try_into()?;

    let result = service::Mutation::create_refill(&conn, refill).await?;

    let id = result.id;
    tracing::info!(
        "{admin} added a new refill \"{}\" - ({} -> {})",
        id,
        result.price,
        result.credit
    );

    Ok((StatusCode::CREATED, id.to_string()).into_response())
}
