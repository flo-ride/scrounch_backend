//! This module contains the route handler for retrieving refill information.

use crate::{
    error::AppError, models::utils::pagination::Pagination, routes::utils::openapi::REFILL_TAG,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use entity::response::refill::{RefillListResponse, RefillResponse, RefillResponseError};
use extractor::profile::admin::Admin;
use migration::IntoCondition;
use sea_orm::ColumnTrait;
use service::Connection;

/// Handles the request to fetch a refill by its unique identifier.
///
/// - **Path Parameters**:  
///   `id` (UUID): The database ID of the refill to retrieve.
///
/// - **Response Codes**:  
///   - `200 OK`: The refill was successfully retrieved.
///   - `404 Not Found`: The refill doesn't exist.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   If the refill is disabled, only an admin can retrieve it.
#[utoipa::path(
    get,
    path = "/{id}", 
    tag = REFILL_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "The database ID of the refill to retrieve."),
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 404, description = "The refill doesn't exist, or is disabled and the requester is not an admin."), 
        (status = 200, description = "The refill was successfully retrieved.", body = RefillResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_refill(
    admin: Option<Admin>,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<Json<RefillResponse>, AppError> {
    let result = service::Query::find_refill_by_id(&conn, id).await?;

    match result {
        Some(refill) => {
            if refill.disabled && admin.is_none() {
                return Err(AppError::NotFound(format!(
                    "The refill with id: {id} doesn't exist"
                )));
            }
            Ok(Json(refill.try_into()?))
        }
        None => Err(AppError::NotFound(format!(
            "The refill with id: {id} doesn't exist"
        ))),
    }
}

/// Handles the request to retrieve a paginated list of refills.
///
/// - **Query Parameters**:  
///   - `page` (Optional, u64): The page index, default is 0.
///   - `per_page` (Optional, u64): The number of refill per page, default is 20.
///
/// - **Response Codes**:  
///   - `200 OK`: Successfully retrieved a list of refills.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   Only Admin can view disabled refill
#[utoipa::path(
    get,
    path = "",
    tag = REFILL_TAG,
    params(
        Pagination
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 200, description = "Successfully retrieved a list of refills.", body = RefillListResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_all_refills(
    admin: Option<Admin>,
    Query(pagination): Query<Pagination>,
    State(conn): State<Connection>,
) -> Result<Json<RefillListResponse>, AppError> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);

    let condition = if admin.is_some() {
        service::every_condition().into_condition()
    } else {
        sea_orm::Condition::any().add(entity::models::refill::Column::Disabled.eq(false))
    };

    let result =
        service::Query::list_refills_with_condition(&conn, condition.clone(), page, per_page)
            .await?;

    let total_page =
        (service::Query::count_refills_with_condition(&conn, condition).await? / per_page) + 1;

    let refills: Result<Vec<_>, RefillResponseError> =
        result.into_iter().map(TryInto::try_into).collect();
    Ok(Json(RefillListResponse {
        current_page: page,
        total_page,
        refills: refills?,
    }))
}
