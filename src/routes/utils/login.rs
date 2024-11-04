//! Utility functions related to login and authentication.
//!
//! This module provides helper functions and utilities specifically for handling
//! login-related operations in the `scrounch_backend` application. These can include
//! functions for managing login sessions, processing user credentials, and handling
//! redirections after successful authentication.
//!

use super::openapi::USER_TAG;
use crate::{error::AppError, state::AppState};
use axum::{extract::State, response::IntoResponse};
use entity::models::sea_orm_active_enums::Currency;
use entity::models::user::{self};
use extractor::profile::oidc_user::OidcUser;
use sea_orm::ActiveValue::Set;
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
    tag = USER_TAG,
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
            user::ActiveModel {
                id: Set(id),
                username: Set(user.username),
                name: Set(user.name),
                email: Set(user.email),
                is_admin: Set(is_admin),
                balance_currency: Set(Currency::Epicoin),
                ..Default::default()
            },
        )
        .await?;
    } else {
        service::Mutation::update_user_last_access_time(&conn, user.id).await?;
    }

    Ok(axum::response::Redirect::to(&state.arguments.frontend_url))
}
