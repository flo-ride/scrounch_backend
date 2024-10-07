//! Route handler for fetching the current user's information.
//!
//! This module provides a handler for the `/me` endpoint, which retrieves the
//! details of the currently authenticated user. It is typically used in contexts
//! where user-specific information needs to be displayed or updated.
use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

/// Handles the `/me` route, returning the current user's information if authenticated.
///
/// This function checks if a user is authenticated using the optional `User`.
/// If the user is logged in, their information (ID, username, email, etc...) is returned as a
/// JSON response. If not logged in, it returns a `204 No Content` response, indicating
/// the user is not authenticated.
#[utoipa::path(
        get,
        path = "/me",
        responses(
            (status = 200, description = "You're logged in", body = User),
            (status = 204, description = "You're not logged in")
        )
    )]
pub async fn get_me(user: Option<crate::models::profile::user::User>) -> impl IntoResponse {
    if let Some(user) = user {
        (StatusCode::OK, json!(user).to_string()).into_response()
    } else {
        (StatusCode::NO_CONTENT, "You're not logged in").into_response()
    }
}
