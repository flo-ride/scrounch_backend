//! This module contains the route handler for retrieving product information.

use crate::utils::openapi::PRODUCT_TAG;
use axum::{
    Json,
    extract::{Path, State},
};
use axum_extra::extract::Query;
use entity::{
    error::AppError,
    models::product::{ProductFilterQuery, ProductSortQuery},
    response::product::{ProductListResponse, ProductResponse, ProductResponseError},
};
use extractor::{profile::admin::Admin, query::Pagination};
use service::Connection;

/// Handles the request to fetch a product by its unique identifier.
///
/// - **Path Parameters**:  
///   `id` (UUID): The database ID of the product to retrieve.
///
/// - **Response Codes**:  
///   - `200 OK`: The product was successfully retrieved.
///   - `404 Not Found`: The product doesn't exist, or is disabled and the requester is not an admin.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   If the product is hidden, only an admin can retrieve it.
#[utoipa::path(get, path = "/{id}", 
    tag = PRODUCT_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "The database ID of the product to retrieve."),
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 404, description = "The product doesn't exist, or is disabled and the requester is not an admin."), 
        (status = 200, description = "The product was successfully retrieved.", body = ProductResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_product(
    admin: Option<Admin>,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<Json<ProductResponse>, AppError> {
    let result = service::Query::find_product_by_id(&conn, id).await?;

    match result {
        Some(product) => {
            if product.hidden && admin.is_none() {
                return Err(AppError::NotFound(format!(
                    "The product with id: {id} doesn't exist"
                )));
            };

            Ok(Json(product.try_into()?))
        }
        None => Err(AppError::NotFound(format!(
            "The product with id: {id} doesn't exist"
        ))),
    }
}

/// Handles the request to retrieve a paginated list of products.
///
/// - **Query Parameters**:  
///   - `page` (Optional, u64): The page index, default is 0.
///   - `per_page` (Optional, u64): The number of products per page, default is 20.
///
/// - **Response Codes**:  
///   - `200 OK`: Successfully retrieved a list of products.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   Non-admin users will only see products that are not hidden.
#[utoipa::path(
    get,
    path = "",
    tag = PRODUCT_TAG,
    params(
        Pagination,
        ProductFilterQuery,
        ProductSortQuery
    ),
    responses(
       (status = 500, description = "An internal error, most likely related to the database, occurred."), 
       (status = 400, description = "The request is improperly formatted."), 
       (status = 200, description = "Successfully retrieved a list of products.", body = ProductListResponse)
    )
)]
pub async fn get_all_products(
    admin: Option<Admin>,
    Query(pagination): Query<Pagination>,
    Query(mut filter): Query<ProductFilterQuery>,
    Query(sort): Query<ProductSortQuery>,
    State(conn): State<Connection>,
) -> Result<Json<ProductListResponse>, AppError> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);

    // Only Admin can view non purchasable/hidden product
    if admin.is_none() {
        filter.purchasable_eq = Some(true);
        filter.purchasable_neq = None;

        filter.hidden_eq = Some(false);
        filter.hidden_neq = None;
    }

    let result =
        service::Query::list_products_with_condition(&conn, filter.clone(), sort, page, per_page)
            .await?;

    let total_products = service::Query::count_products_with_condition(&conn, filter).await?;
    let total_page = ((total_products.max(1) - 1) / per_page) + 1;

    let products = result
        .into_iter()
        .map(|x| x.try_into())
        .collect::<Result<_, ProductResponseError>>()?;
    Ok(Json(ProductListResponse {
        current_page: page,
        total_page,
        products,
    }))
}
