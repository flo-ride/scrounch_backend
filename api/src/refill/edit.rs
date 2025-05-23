//! Route for editing an existing refill in the store.

use crate::utils::openapi::REFILL_TAG;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::{
    error::{AppError, ErrorResponse},
    models::refill::ActiveModel,
    request::refill::EditRefillRequest,
};
use extractor::profile::admin::Admin;
use service::Connection;

/// Edit an existing refill by ID in the store.
///
/// The admin can change attributes such as the name, amont_in_euro, amont_in_epicoin of the refill.
///
/// Returns an error if the refill doesn't exist, if there is a validation issue, or if a database or S3 operation fails.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = REFILL_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "refill database id to edit refill for"),
    ),
    request_body(content = EditRefillRequest, content_type = "application/json"), 
    responses(
       (status = 500, description = "An internal error occured, probably database related"), 
       (status = 400, description = "Your request is not correctly formatted", body = ErrorResponse), 
       (status = 200, description = "The refill is correctly edited")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn edit_refill(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
    Json(edit_refill): Json<EditRefillRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_refill_by_id(&conn, id).await?;

    match result {
        Some(_existing_refill) => {
            let edit_refill: ActiveModel = edit_refill.try_into()?;

            let result = service::Mutation::update_refill(&conn, id, edit_refill).await?;

            log::info!(
                "{admin} successfully edited refill \"{}\" - {:?}",
                id,
                result
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The refill with id: {id} doesn't exist"
        ))),
    }
}
