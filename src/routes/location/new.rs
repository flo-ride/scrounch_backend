//! This module contains the route handler for creating a new location.
//!
//! The handler will be accessible via a POST request to the `/location` endpoint.
//! It allows for the creation of new location entries in the database.
//! Admin privileges are required to access this route.

use crate::{error::AppError, routes::utils::openapi::LOCATION_TAG};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::{models::location::ActiveModel, request::location::NewLocationRequest};
use extractor::profile::admin::Admin;
use service::Connection;

/// Handler for creating a new location.
///
/// This function allows an admin to create a new location by sending a POST request to the `/location` endpoint.
/// The new location is validated and stored in the database. The image associated with the location is checked in S3 storage.
///
/// - **Admin privileges** are required to access this route.
/// - Returns a `201 Created` status upon successful creation along with the location's ID.
///
/// Path: `/location`
///
/// - **Request Body:** Expects a `NewLocation` JSON object.
/// - **Responses:**
///     - 500: Internal server error (likely database related).
///     - 400: Bad request (invalid input data).
///     - 201: Successfully created a new location, returns the new location's ID as a string.
#[utoipa::path(
    post,
    path = "", 
    tag = LOCATION_TAG,
    request_body(content = NewLocationRequest, content_type = "application/json"), 
    responses(
       (status = 500, description = "An internal error, most likely related to the database, occurred."), 
       (status = 400, description = "The request is improperly formatted."), 
       (status = 201, description = "Successfully created a new location, returns the new location's ID as a string.", body = uuid::Uuid)
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn post_new_location(
    admin: Admin,
    State(conn): State<Connection>,
    Json(location): Json<NewLocationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let location_model: ActiveModel = location.try_into()?;

    let result = service::Mutation::create_location(&conn, location_model).await?;

    let id = result.id;

    tracing::info!(
        "{admin} added a new location {} \"{}\" - {:?}",
        id,
        result.name,
        result
    );

    Ok((StatusCode::CREATED, id.to_string()).into_response())
}
