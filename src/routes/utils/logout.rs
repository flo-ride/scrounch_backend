//! Utility functions related to user logout.
//!
//! This module provides helper functions and utilities for managing user logout
//! operations in the `scrounch_backend` application. Functions here handle user
//! session invalidation, redirection after logout, and any other logout-related
//! tasks.

use std::str::FromStr;

use axum::{extract::State, http::Uri, response::IntoResponse};
use axum_oidc::OidcRpInitiatedLogout;

/// Handles the logout process by initiating a logout request with the OIDC provider
///
/// This function manages user logout by initiating an OIDC provider-initiated logout
/// and then redirecting the user to the frontend base URL.
//TODO:: Add utoipa responses
#[utoipa::path(get, path = "/logout")]
pub async fn get_logout(
    State(arguments): State<crate::cli::Arguments>,
    logout: OidcRpInitiatedLogout,
) -> impl IntoResponse {
    let url = Uri::from_str(&arguments.frontend_base_url)
        .expect("Frontend url shoul'd be correctyl formatted");
    logout.with_post_logout_redirect(url)
}
