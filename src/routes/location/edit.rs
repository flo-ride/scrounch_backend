//! Route for editing an existing location

use crate::{error::AppError, models::profile::admin::Admin, routes::utils::openapi::LOCATION_TAG};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{models::location::ActiveModel, request::location::EditLocationRequest};
use service::Connection;

/// Edit an existing location by ID.
///
/// Returns an error if the location doesn't exist, if there is a validation issue, or if a database.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = LOCATION_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "location database id to edit location for"),
    ),
    request_body(content = EditLocationRequest, content_type = "application/json"), 
    responses(
        (status = 500, description = "An internal error occured, probably database related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The location is correctly edited")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn edit_location(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
    Json(edit_location): Json<EditLocationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_location_by_id(&conn, id).await?;

    match result {
        Some(existing_location) => {
            let location_model: ActiveModel = edit_location.try_into()?;

            let result = service::Mutation::update_location(&conn, id, location_model).await?;

            tracing::info!(
                "{admin} successfully edited location {} \"{}\" - {:?}",
                existing_location.name,
                id,
                result
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The location with id: {id} doesn't exist"
        ))),
    }
}
