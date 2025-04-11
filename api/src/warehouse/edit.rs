//! Route for editing an existing warehouse in the store.

use crate::utils::openapi::WAREHOUSE_TAG;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::{error::AppError, models::warehouse, request::warehouse::EditWarehouseRequest};
use extractor::profile::admin::Admin;
use service::Connection;

/// Edit an existing warehouse by ID in the store.
///
/// The admin can change attributes such as the name, price, quantity, or image of the warehouse.
/// If the warehouse image is changed, the old image will be deleted from S3 storage.
///
/// Returns an error if the warehouse doesn't exist, if there is a validation issue, or if a database or S3 operation fails.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = WAREHOUSE_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "Warehouse database id to edit warehouse for"),
    ),
    request_body(content = EditWarehouseRequest, content_type = "application/json"), 
    responses(
        (status = 500, description = "An internal error occured, probably database related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The warehouse is correctly edited")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn edit_warehouse(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
    Json(edit_warehouse): Json<EditWarehouseRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_warehouse_by_id(&conn, id).await?;

    match result {
        Some(_existing_warehouse) => {
            let edit_warehouse_model: warehouse::ActiveModel = edit_warehouse.clone().try_into()?;

            let result =
                service::Mutation::update_warehouse(&conn, id, edit_warehouse_model).await?;

            log::info!(
                "{admin} successfully edited warehouse \"{}\" - {:?}",
                id,
                result
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The warehouse with id: {id} doesn't exist"
        ))),
    }
}
