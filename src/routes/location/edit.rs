//! Route for editing an existing location

use crate::{
    error::AppError,
    models::{profile::admin::Admin, request::location::EditLocation},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::models::location::Model as Location;
use service::Connection;

/// Edit an existing location by ID.
///
/// Returns an error if the location doesn't exist, if there is a validation issue, or if a database.
#[utoipa::path(put, path = "/location/{id}",
               params(
                   ("id" = uuid::Uuid, Path, description = "location database id to edit location for"),
                ),
               request_body(content = EditLocation, content_type = "application/json"), 
               responses(
                   (status = 500, description = "An internal error occured, probably database related"), 
                   (status = 400, description = "Your request is not correctly formatted"), 
                   (status = 200, description = "The location is correctly edited")
                )
               )]
pub async fn edit_location(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
    Json(new_location): Json<EditLocation>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_location_by_id(&conn, id).await?;

    match result {
        Some(existing_location) => {
            service::Mutation::update_location(
                &conn,
                id,
                Location {
                    id,
                    name: match new_location.name.clone() {
                        None => existing_location.name.clone(),
                        Some(name) => {
                            let max_length = 32;
                            if name.is_empty() {
                                return Err(AppError::BadOption(
                                    "Name cannot be empty".to_string(),
                                ));
                            }
                            if name.len() > max_length {
                                return Err(AppError::BadOption(format!(
                                    "Name cannot be longer than {max_length}: {name}",
                                )));
                            }
                            name
                        }
                    },
                    category: new_location.category.map(Into::into),
                    creation_time: existing_location.creation_time,
                    disabled: match new_location.disabled {
                        Some(disabled) => disabled,
                        None => existing_location.disabled,
                    },
                },
            )
            .await?;

            tracing::info!(
                "Admin {} \"{}\" successfully edited location {} \"{}\" - {:?}",
                admin.name,
                admin.id,
                existing_location.name,
                id,
                new_location
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The location with id: {id} doesn't exist"
        ))),
    }
}
