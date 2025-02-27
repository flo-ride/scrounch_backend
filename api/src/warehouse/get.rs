//! This module contains the route handler for retrieving warehouse information.

use crate::utils::openapi::WAREHOUSE_TAG;
use axum::{
    extract::{Path, State},
    Json,
};
use axum_extra::extract::Query;
use entity::{
    error::AppError,
    models::{
        warehouse::{WarehouseFilterQuery, WarehouseSortQuery},
        warehouse_product::{Warehouse_productFilterQuery, Warehouse_productSortQuery},
    },
    response::warehouse::{
        WarehouseListResponse, WarehouseProductResponse, WarehouseProductResponseError,
        WarehouseProductsListResponse, WarehouseResponse,
    },
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

/// Retrieves a specific product from a warehouse.
///
/// This endpoint allows an administrator to fetch details about a product
/// stored in a specified warehouse. If the warehouse or product does not exist,
/// or if the warehouse is disabled and the requester lacks administrative privileges,
/// an appropriate error response is returned.
#[utoipa::path(get, path = "/{warehouse_id}/product/{product_id}", 
    tag = WAREHOUSE_TAG,
    params(
        ("warehouse_id" = uuid::Uuid, Path, description = "The database ID of the warehouse to retrieve."),
        ("product_id" = uuid::Uuid, Path, description = "The database ID of the product to retrieve."),
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 404, description = "The warehouse doesn't exist, or is disabled and the requester is not an admin."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 200, description = "The warehouse was successfully retrieved.", body = WarehouseProductResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_warehouse_product(
    _admin: Admin,
    Path((warehouse_id, product_id)): Path<(uuid::Uuid, uuid::Uuid)>,
    State(conn): State<Connection>,
) -> Result<Json<WarehouseProductResponse>, AppError> {
    let result =
        service::Query::find_warehouse_product_by_id(&conn, warehouse_id, product_id).await?;

    match result {
        Some(warehouse_product) => Ok(Json(warehouse_product.try_into()?)),
        None => Err(AppError::NotFound(format!(
            "Could not find warehouse \"{warehouse_id}\" / product \"{product_id}\" combination"
        ))),
    }
}

/// Fetches all warehouse products with pagination, filtering, and sorting options.
///
/// # Parameters
/// - `warehouse_id`: The unique identifier of the warehouse.
/// - `pagination`: Query parameters for pagination, including `page` and `per_page`.
/// - `filter`: Query parameters for filtering the products (e.g., by category, availability).
/// - `sort`: Query parameters for sorting the products (e.g., by price or name).
/// - `conn`: The database connection used to query the data.
///
/// # Returns
/// - `200 OK`: A paginated list of products for the warehouse, including the current page and total pages.
/// - `500 Internal Server Error`: An internal error occurs, possibly related to the database.
/// - `400 Bad Request`: Invalid query parameters (pagination, filtering, or sorting).
#[utoipa::path(get, path = "/{warehouse_id}/product", 
    tag = WAREHOUSE_TAG,
    params(
        ("warehouse_id" = uuid::Uuid, Path, description = "The database ID of the warehouse to retrieve."),
        Pagination,
        Warehouse_productFilterQuery,
        Warehouse_productSortQuery,
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 404, description = "The warehouse doesn't exist, or is disabled and the requester is not an admin."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 200, description = "The warehouse was successfully retrieved.", body = WarehouseProductsListResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_all_warehouse_products(
    _admin: Admin,
    Path(warehouse_id): Path<uuid::Uuid>,
    Query(pagination): Query<Pagination>,
    Query(filter): Query<Warehouse_productFilterQuery>,
    Query(sort): Query<Warehouse_productSortQuery>,
    State(conn): State<Connection>,
) -> Result<Json<WarehouseProductsListResponse>, AppError> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);

    let result = service::Query::list_warehouse_products(
        &conn,
        warehouse_id,
        filter.clone(),
        sort,
        page,
        per_page,
    )
    .await?;

    let total_warehouses =
        service::Query::count_warehouse_products(&conn, warehouse_id, filter).await?;
    let total_page = ((total_warehouses.max(1) - 1) / per_page) + 1;

    let products = result
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<_, WarehouseProductResponseError>>()?;
    Ok(Json(WarehouseProductsListResponse {
        current_page: page,
        total_page,
        products,
    }))
}
