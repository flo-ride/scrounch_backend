//! This module contains the route handler for creating a new warehouse.
//!
//! The handler will be accessible via a POST request to the `/warehouse` endpoint.
//! It allows for the creation of new warehouse entries in the database.
//! Admin privileges are required to access this route.

use crate::utils::openapi::WAREHOUSE_TAG;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::{
    error::{AppError, ErrorResponse},
    models::warehouse::ActiveModel,
    request::warehouse::{NewWarehouseRequest, WarehouseRequestError},
};
use extractor::profile::admin::Admin;
use service::Connection;

/// Handler for creating a new warehouse.
///
/// This function allows an admin to create a new warehouse by sending a POST request to the `/warehouse` endpoint.
/// The new warehouse is validated and stored in the database. The image associated with the warehouse is checked in S3 storage.
///
/// - **Admin privileges** are required to access this route.
/// - Returns a `201 Created` status upon successful creation along with the warehouse's ID.
///
/// Path: `/warehouse`
///
/// - **Request Body:** Expects a `NewWarehouse` JSON object.
/// - **Responses:**
///     - 500: Internal server error (likely database related).
///     - 400: Bad request (invalid input data).
///     - 201: Successfully created a new warehouse, returns the new warehouse's ID as a string.
#[utoipa::path(
    post,
    path = "", 
    tag = WAREHOUSE_TAG,
    request_body(content = NewWarehouseRequest, content_type = "application/json"), 
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted.", body = ErrorResponse), 
        (status = 201, description = "Successfully created a new warehouse, returns the new warehouse's ID as a string.", body = uuid::Uuid)
    )
)]
pub async fn post_new_warehouse(
    admin: Admin,
    State(conn): State<Connection>,
    Json(warehouse): Json<NewWarehouseRequest>,
) -> Result<impl IntoResponse, AppError> {
    let warehouse_model: ActiveModel = warehouse.clone().try_into()?;

    // Verifiy that every warehouse exist before mutating anything
    if let Some(parent) = warehouse.parent {
        service::Query::find_warehouse_by_id(&conn, parent)
            .await?
            .ok_or(WarehouseRequestError::ParentDoesntExist(parent))?;
    }

    let result = service::Mutation::create_warehouse(&conn, warehouse_model).await?;
    let id = result.id;

    log::info!(
        "{admin} added a new warehouse {} for {} - {:?}",
        id,
        result.id,
        result
    );

    Ok((StatusCode::CREATED, id.to_string()).into_response())
}
