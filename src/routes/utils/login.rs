//! Utility functions related to login and authentication.
//!
//! This module provides helper functions and utilities specifically for handling
//! login-related operations in the `scrounch_backend` application. These can include
//! functions for managing login sessions, processing user credentials, and handling
//! redirections after successful authentication.
//!

use axum::{extract::State, response::IntoResponse};

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
pub async fn get_login(State(arguments): State<crate::cli::Arguments>) -> impl IntoResponse {
    axum::response::Redirect::to(&arguments.frontend_url)
}
