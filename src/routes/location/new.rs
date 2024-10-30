//! This module contains the route handler for creating a new location.
//!
//! The handler will be accessible via a POST request to the `/location` endpoint.
//! It allows for the creation of new location entries in the database.
//! Admin privileges are required to access this route.

use crate::models::profile::admin::Admin;
use crate::{error::AppError, models::request::location::NewLocation};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::location::Model as Location;
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
#[utoipa::path(post, path = "/location", 
               request_body(content = NewLocation, content_type = "application/json"), 
               responses(
                   (status = 500, description = "An internal error, most likely related to the database, occurred."), 
                   (status = 400, description = "The request is improperly formatted."), 
                   (status = 201, description = "Successfully created a new location, returns the new location's ID as a string.", body = uuid::Uuid)
                )
               )]
pub async fn post_new_location(
    admin: Admin,
    State(conn): State<Connection>,
    Json(location): Json<NewLocation>,
) -> Result<impl IntoResponse, AppError> {
    let id = uuid::Uuid::new_v4();
    service::Mutation::create_location(
        &conn,
        Location {
            id,
            name: {
                let name = location.name.clone();
                let max_length = 32;
                if name.is_empty() {
                    return Err(AppError::BadOption("Name cannot be empty".to_string()));
                }
                if name.len() > max_length {
                    return Err(AppError::BadOption(format!(
                        "Name cannot be longer than {max_length}: {name}",
                    )));
                }
                name
            },
            category: location.category.map(Into::into),
            disabled: false,
            creation_time: chrono::offset::Local::now().into(),
        },
    )
    .await?;

    tracing::info!(
        "Admin {} \"{}\" added a new location {} \"{}\" - {:?}",
        admin.name,
        admin.id,
        location.name,
        id,
        location
    );

    Ok((StatusCode::CREATED, id.to_string()).into_response())
}
