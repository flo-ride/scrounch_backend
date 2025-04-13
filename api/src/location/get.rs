//! This module contains the route handler for retrieving location information.

use crate::utils::openapi::LOCATION_TAG;
use axum::{
    Json,
    extract::{Path, State},
};
use axum_extra::extract::Query;
use entity::{
    error::AppError,
    models::location::{LocationFilterQuery, LocationSortQuery},
    response::location::{LocationListResponse, LocationResponse},
};
use extractor::{profile::admin::Admin, query::Pagination};
use service::Connection;

/// Handles the request to fetch a location by its unique identifier.
///
/// - **Path Parameters**:  
///   `id` (UUID): The database ID of the location to retrieve.
///
/// - **Response Codes**:  
///   - `200 OK`: The location was successfully retrieved.
///   - `404 Not Found`: The location doesn't exist.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   If the location is hidden, only an admin can retrieve it.
#[utoipa::path(
    get,
    path = "/{id}", 
    tag = LOCATION_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "The database ID of the location to retrieve."),
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 404, description = "The location doesn't exist, or is disabled and the requester is not an admin."), 
        (status = 200, description = "The location was successfully retrieved.", body = LocationResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_location(
    admin: Option<Admin>,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<Json<LocationResponse>, AppError> {
    let result = service::Query::find_location_by_id(&conn, id).await?;

    match result {
        Some(location) => {
            if location.hidden && admin.is_none() {
                return Err(AppError::NotFound(format!(
                    "The location with id: {id} doesn't exist"
                )));
            }
            Ok(Json(location.into()))
        }
        None => Err(AppError::NotFound(format!(
            "The location with id: {id} doesn't exist"
        ))),
    }
}

/// Handles the request to retrieve a paginated list of locations.
///
/// - **Query Parameters**:  
///   - `page` (Optional, u64): The page index, default is 0.
///   - `per_page` (Optional, u64): The number of location per page, default is 20.
///
/// - **Response Codes**:  
///   - `200 OK`: Successfully retrieved a list of locations.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   Only Admin can view hidden location
#[utoipa::path(
    get,
    path = "",
    tag = LOCATION_TAG,
    params(
        Pagination,
        LocationFilterQuery,
        LocationSortQuery,
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 200, description = "Successfully retrieved a list of locations.", body = LocationListResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_all_locations(
    admin: Option<Admin>,
    Query(pagination): Query<Pagination>,
    Query(mut filter): Query<LocationFilterQuery>,
    Query(sort): Query<LocationSortQuery>,
    State(conn): State<Connection>,
) -> Result<Json<LocationListResponse>, AppError> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);

    if admin.is_none() {
        filter.hidden_eq = Some(false);
        filter.hidden_eq = None;
    }

    let result =
        service::Query::list_locations_with_condition(&conn, filter.clone(), sort, page, per_page)
            .await?;

    let total_locations = service::Query::count_locations_with_condition(&conn, filter).await?;
    let total_page = ((total_locations.max(1) - 1) / per_page) + 1;

    let locations = result.into_iter().map(Into::into).collect();
    Ok(Json(LocationListResponse {
        current_page: page,
        total_page,
        locations,
    }))
}
