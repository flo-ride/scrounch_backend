//! This module contains the route handler for retrieving warehouse information.

use crate::utils::openapi::WAREHOUSE_TAG;
use axum::{
    extract::{Path, State},
    Json,
};
use axum_extra::extract::Query;
use entity::{
    error::AppError,
    models::warehouse::{WarehouseFilterQuery, WarehouseSortQuery},
    response::warehouse::{WarehouseListResponse, WarehouseResponse},
};
use extractor::{profile::admin::Admin, query::Pagination};
use service::Connection;

/// Handles the request to fetch a warehouse by its unique identifier.
///
/// - **Path Parameters**:  
///   `id` (UUID): The database ID of the warehouse to retrieve.
///
/// - **Response Codes**:  
///   - `200 OK`: The warehouse was successfully retrieved.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `404 Not Found`: The warehouse doesn't exist, or is disabled and the requester is not an admin.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   If the warehouse is hidden, only an admin can retrieve it.
#[utoipa::path(get, path = "/{id}", 
    tag = WAREHOUSE_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "The database ID of the warehouse to retrieve."),
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 404, description = "The warehouse doesn't exist, or is disabled and the requester is not an admin."), 
        (status = 200, description = "The warehouse was successfully retrieved.", body = WarehouseResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_warehouse(
    _admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<Json<WarehouseResponse>, AppError> {
    let result = service::Query::find_warehouse_by_id(&conn, id).await?;

    match result {
        Some(warehouse) => Ok(Json(warehouse.into())),
        None => Err(AppError::NotFound(format!(
            "The warehouse with id: {id} doesn't exist"
        ))),
    }
}

/// Handles the request to retrieve a paginated list of warehouses.
///
/// - **Query Parameters**:  
///   - `page` (Optional, u64): The page index, default is 0.
///   - `per_page` (Optional, u64): The number of warehouses per page, default is 20.
///
/// - **Response Codes**:  
///   - `200 OK`: Successfully retrieved a list of warehouses.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   Non-admin users will only see warehouses that are not hidden.
#[utoipa::path(
    get,
    path = "",
    tag = WAREHOUSE_TAG,
    params(
        Pagination,
        WarehouseFilterQuery,
        WarehouseSortQuery
    ),
    responses(
       (status = 500, description = "An internal error, most likely related to the database, occurred."), 
       (status = 400, description = "The request is improperly formatted."), 
       (status = 200, description = "Successfully retrieved a list of warehouses.", body = WarehouseListResponse)
    )
)]
pub async fn get_all_warehouses(
    _admin: Admin,
    Query(pagination): Query<Pagination>,
    Query(filter): Query<WarehouseFilterQuery>,
    Query(sort): Query<WarehouseSortQuery>,
    State(conn): State<Connection>,
) -> Result<Json<WarehouseListResponse>, AppError> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);

    let result =
        service::Query::list_warehouses_with_condition(&conn, filter.clone(), sort, page, per_page)
            .await?;

    let total_warehouses = service::Query::count_warehouses_with_condition(&conn, filter).await?;
    let total_page = ((total_warehouses.max(1) - 1) / per_page) + 1;

    let warehouses = result.into_iter().map(Into::into).collect();
    Ok(Json(WarehouseListResponse {
        current_page: page,
        total_page,
        warehouses,
    }))
}
