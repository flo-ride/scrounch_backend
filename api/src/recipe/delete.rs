//! This module defines the API endpoint to delete a recipe by its ID.
//!
//! Only an admin can delete a recipe.

use crate::utils::openapi::RECIPE_TAG;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::error::AppError;
use extractor::profile::admin::Admin;
use service::Connection;

/// Deletes a recipe by its database ID.
///
/// The recipe is not fully removed but marked as disabled in the database.
/// Only an admin can perform this action.
///
/// - **Path Parameters:**
///   - `id`: The unique ID of the recipe in the database.
///
/// - **Responses:**
///   - `500`: Internal error, likely related to the database.
///   - `400`: The request format is invalid.
///   - `200`: The recipe has been successfully disabled.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = RECIPE_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "Recipe database id to delete recipe for"),
    ),
    responses(
        (status = 500, description = "An internal error occured, probably databse related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The recipe is disabled")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn delete_recipe(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_recipe_by_id(&conn, id).await?;

    match result {
        Some(recipe) => {
            service::Mutation::delete_recipe(&conn, id).await?;

            log::info!("{admin} just deleted the recipe  \"{}\" - {:?}", id, recipe);

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The recipe with id: {id} doesn't exist"
        ))),
    }
}
