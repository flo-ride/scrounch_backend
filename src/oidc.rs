//! OpenID Connect (OIDC) utilities and configuration.
//!
//! This module provides the necessary utilities for setting up and managing OpenID Connect (OIDC)
//! authentication in the `scrounch_backend` application. It includes functions for creating an OIDC
//! client, managing sessions, and handling authentication layers in the Axum framework.

use std::str::FromStr;

use axum::response::IntoResponse;
use axum_oidc::{error::MiddlewareError, EmptyAdditionalClaims};

/// Provides a session layer for managing user sessions.
///
/// This function returns a session layer based on an in-memory store. The session layer
/// is used to manage user authentication sessions in the application, allowing the
/// server to store session data such as authentication tokens.
pub fn memory_session_layer(
    duration: std::time::Duration,
) -> tower_sessions::SessionManagerLayer<tower_sessions::MemoryStore> {
    let session_store = tower_sessions::MemoryStore::default();
    tower_sessions::SessionManagerLayer::new(session_store)
        .with_same_site(tower_sessions::cookie::SameSite::None)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::seconds_f64(duration.as_secs_f64()),
        ))
}

#[cfg(feature = "cache")]
pub fn cache_session_layer(
    pool: fred::clients::RedisPool,
    duration: std::time::Duration,
) -> tower_sessions::SessionManagerLayer<
    tower_sessions_redis_store::RedisStore<fred::clients::RedisPool>,
> {
    let session_store = tower_sessions_redis_store::RedisStore::new(pool);
    tower_sessions::SessionManagerLayer::new(session_store)
        .with_same_site(tower_sessions::cookie::SameSite::None)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::seconds_f64(duration.as_secs_f64()),
        ))
}

/// Creates an OIDC client for handling authentication.
///
/// This function initializes an OpenID Connect client
pub async fn get_oidc_client(
    arguments: &crate::Arguments,
) -> Result<axum_oidc::OidcAuthLayer<EmptyAdditionalClaims>, axum_oidc::error::Error> {
    let backend_base_url =
        axum::http::Uri::from_str(&arguments.backend_url).expect("BACKEND_BASE_URL is not valid");
    let issuer = arguments.openid_issuer.to_owned();
    let client_id = arguments.openid_client_id.to_owned();
    let client_secret = arguments.openid_client_secret.to_owned();

    axum_oidc::OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
        backend_base_url,
        issuer,
        client_id,
        client_secret,
        vec!["email".to_string(), "profile".to_string()],
    )
    .await
}

/// Handles errors encountered in the OpenID Connect (OIDC) middleware.
///
/// This function provides a centralized way to handle errors that occur in the
/// OIDC authentication middleware. It takes a `MiddlewareError`, which may occur
/// during login, token validation, or other OIDC-related operations, and converts
/// it into an appropriate HTTP response.
pub async fn handle_axum_oidc_middleware_error(
    e: MiddlewareError,
) -> axum::http::Response<axum::body::Body> {
    e.into_response()
}
