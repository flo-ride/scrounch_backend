//! This module contains the route handler for retrieving product information.

use crate::{
    error::AppError,
    models::{
        profile::admin::Admin, response::product::ProductResponse, utils::pagination::Pagination,
    },
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use migration::IntoCondition;
use sea_orm::ColumnTrait;
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
///   If the product is disabled, only an admin can retrieve it.
#[utoipa::path(get, path = "/product/{id}", 
               params(
                   ("id" = uuid::Uuid, Path, description = "The database ID of the product to retrieve."),
                ),
                responses(
                   (status = 500, description = "An internal error, most likely related to the database, occurred."), 
                   (status = 400, description = "The request is improperly formatted."), 
                   (status = 404, description = "The product doesn't exist, or is disabled and the requester is not an admin."), 
                   (status = 200, description = "The product was successfully retrieved.", body = ProductResponse)
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
            if product.disabled && admin.is_none() {
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
///   Non-admin users will only see products that are not disabled.
#[utoipa::path(get, path = "/product",
               params(
                   ("page" = Option<u64>, Query, description = "The page index, default is 0"),
                   ("per_page" = Option<u64>, Query, description = "The number of products per page, default is 20"),
                ),
                responses(
                   (status = 500, description = "An internal error, most likely related to the database, occurred."), 
                   (status = 400, description = "The request is improperly formatted."), 
                   (status = 200, description = "Successfully retrieved a list of products.", body = Vec<ProductResponse>)
                )
            )]
pub async fn get_all_products(
    admin: Option<Admin>,
    Query(pagination): Query<Pagination>,
    State(conn): State<Connection>,
) -> Result<Json<Vec<ProductResponse>>, AppError> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);

    let condition = if admin.is_some() {
        service::every_condition().into_condition()
    } else {
        sea_orm::Condition::any().add(entity::product::Column::Disabled.eq(false))
    };

    let result =
        service::Query::list_products_with_condition(&conn, condition, page, per_page).await?;

    let result = result
        .into_iter()
        .map(|x| x.try_into())
        .collect::<Result<_, AppError>>()?;
    Ok(Json(result))
}
