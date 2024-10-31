//! Utility functions related to login and authentication.
//!
//! This module provides helper functions and utilities specifically for handling
//! login-related operations in the `scrounch_backend` application. These can include
//! functions for managing login sessions, processing user credentials, and handling
//! redirections after successful authentication.
//!

use crate::error::AppError;
use crate::{models::profile::oidc_user::OidcUser, state::AppState};
use axum::{extract::State, response::IntoResponse};
use entity::models::user::Model as User;
use service::Connection;

/// Handles the login route by redirecting the user to the frontend.
///
/// This function is responsible for handling login requests. When a user attempts to
/// access the login route, they are redirected to the frontend base URL specified in
/// the application's configuration (command-line arguments or environment variables).
///
#[utoipa::path(
        get,
        path = "/login",
        responses(
            (status = 307, description = "You're not logged in and you should be"),
            (status = 303, description = "You're logged in, now go back to frontend_base_url")
        )
    )]
pub async fn get_login(
    user: OidcUser,
    State(conn): State<Connection>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    if service::Query::find_user_by_id(&conn, user.id)
        .await?
        .is_none()
    {
        let id = user.id;

        let mut is_admin = false;

        // In case no User exist, the first one become an Admin
        if let Ok(0) =
            service::Query::count_users_with_condition(&conn, service::every_condition()).await
        {
            is_admin = true;
        }

        service::Mutation::create_user(
            &conn,
            User {
                id,
                username: user.username,
                name: user.name,
                email: user.email,
                is_admin,
                is_banned: false,
                creation_time: chrono::offset::Local::now().into(),
                last_access_time: chrono::offset::Local::now().into(),
            },
        )
        .await?;
    } else {
        service::Mutation::update_user_last_access_time(&conn, user.id).await?;
    }

    Ok(axum::response::Redirect::to(&state.arguments.frontend_url))
}
