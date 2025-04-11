//! Route handler for fetching the current user's information.
//!
//! This module provides a handler for the `/me` endpoint, which retrieves the
//! details of the currently authenticated user. It is typically used in contexts
//! where user-specific information needs to be displayed or updated.
use crate::utils::openapi::USER_TAG;
use axum::{Json, extract::State};
use entity::{error::AppError, response::user::UserResponse};
use extractor::profile::{oidc_user::OidcUser, user::User};
use service::Connection;

/// Handles the `/me` route, returning the current user's information if authenticated.
///
/// This function checks if a user is authenticated using the optional `User`.
/// If the user is logged in, their information (ID, username, email, etc...) is returned as a
/// JSON response. If not logged in, it returns a `204 No Content` response, indicating
/// the user is not authenticated.
#[utoipa::path(
    get,
    path = "/me",
    tag = USER_TAG,
    responses(
        (status = 200, description = "You're logged in", body = UserResponse),
        (status = 204, description = "You're not logged in")
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_me(
    user: Option<User>,
    oidc_user: Option<OidcUser>,
    State(conn): State<Connection>,
) -> Result<Json<UserResponse>, AppError> {
    if let Some(user) = user {
        let _ = service::Mutation::update_user_last_access_time(&conn, user.id).await;
        Ok(Json(user.into()))
    } else if let Some(oidc_user) = oidc_user {
        // User is probably banned so the handler failed with Forbidden
        let user = service::Query::find_user_by_id(&conn, oidc_user.id).await?;
        match user {
            Some(user) => Ok(Json(user.into())),
            None => Err(AppError::NoContent),
        }
    } else {
        Err(AppError::NoContent)
    }
}
