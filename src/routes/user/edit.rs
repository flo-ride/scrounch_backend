//! Route for editing an existing user

use crate::{
    error::AppError,
    models::{profile::admin::Admin, request::user::EditUser},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::user::Model as User;
use service::Connection;

/// Edit an existing user by ID.
///
/// Returns an error if the user doesn't exist, if there is a validation issue, or if a database.
#[utoipa::path(put, path = "/user/{id}",
               params(
                   ("id" = uuid::Uuid, Path, description = "user database id to edit user for"),
                ),
               request_body(content = EditUser, content_type = "application/json"), 
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
    Json(new_user): Json<EditUser>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_user_by_id(&conn, id).await?;

    match result {
        Some(existing_user) => {
            service::Mutation::update_user(
                &conn,
                id,
                User {
                    id,
                    name: existing_user.name.clone(),
                    email: existing_user.email,
                    username: existing_user.username,
                    is_admin: match new_user.is_admin {
                        Some(is_admin) => is_admin,
                        None => existing_user.is_admin,
                    },
                    creation_time: existing_user.creation_time,
                    last_access_time: existing_user.last_access_time,
                },
            )
            .await?;

            tracing::info!(
                "Admin {} \"{}\" successfully edited user {} \"{}\" - {:?}",
                admin.name,
                admin.id,
                existing_user.name,
                id,
                new_user
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The user with id: {id} doesn't exist"
        ))),
    }
}
