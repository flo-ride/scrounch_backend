//! This module contains the route handler for retrieving user information.

use crate::{
    error::AppError,
    models::{
        profile::{admin::Admin, user::User},
        response::user::{UserListResponse, UserResponse},
        utils::pagination::Pagination,
    },
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use migration::IntoCondition;
use service::Connection;

/// Handles the request to fetch a user by its unique identifier.
///
/// - **Path Parameters**:  
///   `id` (UUID): The database ID of the user to retrieve.
///
/// - **Response Codes**:  
///   - `200 OK`: The user was successfully retrieved.
///   - `404 Not Found`: The user doesn't exist.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   If the user is disabled, only an admin can retrieve it.
#[utoipa::path(get, path = "/user/{id}", 
               params(
                   ("id" = uuid::Uuid, Path, description = "The database ID of the user to retrieve."),
                ),
                responses(
                   (status = 500, description = "An internal error, most likely related to the database, occurred."), 
                   (status = 400, description = "The request is improperly formatted."), 
                   (status = 404, description = "The user doesn't exist, or is disabled and the requester is not an admin."), 
                   (status = 200, description = "The user was successfully retrieved.", body = UserResponse)
                )
            )]
pub async fn get_user(
    user: User,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<Json<UserResponse>, AppError> {
    if user.id != id && !user.is_admin {
        return Err(AppError::Forbidden);
    }

    let result = service::Query::find_user_by_id(&conn, id).await?;

    match result {
        Some(user) => Ok(Json(user.into())),
        None => Err(AppError::NotFound(format!(
            "The user with id: {id} doesn't exist"
        ))),
    }
}

/// Handles the request to retrieve a paginated list of users.
///
/// - **Query Parameters**:  
///   - `page` (Optional, u64): The page index, default is 0.
///   - `per_page` (Optional, u64): The number of user per page, default is 20.
///
/// - **Response Codes**:  
///   - `200 OK`: Successfully retrieved a list of users.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   Only Admin can view others users
#[utoipa::path(get, path = "/user",
               params(
                   ("page" = Option<u64>, Query, description = "The page index, default is 0"),
                   ("per_page" = Option<u64>, Query, description = "The number of user per page, default is 20"),
                ),
                responses(
                   (status = 500, description = "An internal error, most likely related to the database, occurred."), 
                   (status = 400, description = "The request is improperly formatted."), 
                   (status = 200, description = "Successfully retrieved a list of users.", body = UserListResponse)
                )
            )]
pub async fn get_all_users(
    _admin: Admin,
    Query(pagination): Query<Pagination>,
    State(conn): State<Connection>,
) -> Result<Json<UserListResponse>, AppError> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);

    let condition = service::every_condition().into_condition();

    let result =
        service::Query::list_users_with_condition(&conn, condition.clone(), page, per_page).await?;

    let total_page =
        (service::Query::count_users_with_condition(&conn, condition).await? / per_page) + 1;

    let users = result.into_iter().map(|x| x.into()).collect();
    Ok(Json(UserListResponse {
        current_page: page,
        total_page,
        users,
    }))
}
