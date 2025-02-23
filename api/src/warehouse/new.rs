//! This module contains the route handler for creating a new warehouse.
//!
//! The handler will be accessible via a POST request to the `/warehouse` endpoint.
//! It allows for the creation of new warehouse entries in the database.
//! Admin privileges are required to access this route.

use crate::utils::openapi::WAREHOUSE_TAG;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{
    error::{AppError, ErrorResponse},
    models::{warehouse, warehouse_product},
    request::warehouse::{
        NewWarehouseProductRequest, NewWarehouseRequest, WarehouseProductRequestError,
    },
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
    let warehouse_model: warehouse::ActiveModel = warehouse.clone().try_into()?;

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

/// Handles the creation of a new warehouse product.
///
/// This endpoint allows an administrator to associate a product with a warehouse.
/// It verifies the existence of both the warehouse and the product before proceeding
/// with the creation. The function returns a `201 Created` response upon success,
/// or an appropriate error response if the request is invalid or an internal error occurs.
#[utoipa::path(
    post,
    path = "/{warehouse_id}/product/{product_id}", 
    tag = WAREHOUSE_TAG,
    params(
        ("warehouse_id" = uuid::Uuid, Path, description = "The database ID of the warehouse to retrieve."),
        ("product_id" = uuid::Uuid, Path, description = "The database ID of the product to retrieve."),
    ),
    request_body(content = NewWarehouseProductRequest, content_type = "application/json"), 
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 404, description = "Can't find Warehouse or Product"), 
        (status = 400, description = "The request is improperly formatted.", body = ErrorResponse), 
        (status = 201, description = "Successfully created a new warehouse product")
    )
)]
pub async fn post_new_warehouse_product(
    admin: Admin,
    Path((warehouse_id, product_id)): Path<(uuid::Uuid, uuid::Uuid)>,
    State(conn): State<Connection>,
    Json(warehouse_product): Json<NewWarehouseProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result_warehouse = service::Query::find_warehouse_by_id(&conn, warehouse_id).await?;
    let result_product = service::Query::find_product_by_id(&conn, product_id).await?;

    match (result_warehouse, result_product) {
        (Some(_warehouse), Some(_product)) => {
            let warehouse_product_model: warehouse_product::ActiveModel =
                warehouse_product.try_into()?;

            let result = service::Mutation::create_warehouse_product(
                &conn,
                warehouse_id,
                product_id,
                warehouse_product_model,
            )
            .await?;

            log::info!(
        "{admin} added a new warehouse ({warehouse_id}) product ({product_id}) - {result:?}",
    );

            Ok((StatusCode::CREATED, ""))
        }
        (warehouse_option, product_option) => {
            if warehouse_option.is_none() {
                return Err(WarehouseProductRequestError::WarehouseDoesntExist(
                    warehouse_id,
                ))?;
            }

            if product_option.is_none() {
                return Err(WarehouseProductRequestError::ProductDoesntExist(product_id))?;
            }

            Err(AppError::InternalError(
                "Something weird happened, both warehouse & product are here but not too"
                    .to_string(),
            ))
        }
    }
}
