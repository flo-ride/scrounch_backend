//! Route for editing an existing user

use crate::{
    error::AppError,
    models::profile::{admin::Admin, user::User},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::request::user::EditUserRequest;
use service::Connection;

/// Edit an existing user by ID.
///
/// Returns an error if the user doesn't exist, if there is a validation issue, or if a database.
#[utoipa::path(put, path = "/user/{id}",
               params(
                   ("id" = uuid::Uuid, Path, description = "user database id to edit user for"),
                ),
               request_body(content = EditUserRequest, content_type = "application/json"), 
               responses(
                   (status = 500, description = "An internal error occured, probably database related"), 
                   (status = 400, description = "Your request is not correctly formatted"), 
                   (status = 200, description = "The user is correctly edited")
                )
               )]
pub async fn edit_user(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
    Json(edit_user): Json<EditUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_user_by_id(&conn, id).await?;

    match result {
        Some(existing_user) => {
            let result = service::Mutation::update_user(&conn, id, edit_user).await?;

            tracing::info!(
                "{admin} successfully edited {} - {:?}",
                Into::<User>::into(existing_user),
                result
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The user with id: {id} doesn't exist"
        ))),
    }
}
