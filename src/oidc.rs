//! OpenID Connect (OIDC) utilities and configuration.
//!
//! This module provides the necessary utilities for setting up and managing OpenID Connect (OIDC)
//! authentication in the `scrounch_backend` application. It includes functions for creating an OIDC
//! client, managing sessions, and handling authentication layers in the Axum framework.

use std::str::FromStr;

use axum::{extract::FromRequestParts, response::IntoResponse};
use axum_oidc::{
    error::{ExtractorError, MiddlewareError},
    EmptyAdditionalClaims, OidcClaims,
};

/// Provides a session layer for managing user sessions.
///
/// This function returns a session layer based on an in-memory store. The session layer
/// is used to manage user authentication sessions in the application, allowing the
/// server to store session data such as authentication tokens.
pub fn memory_session_layer() -> tower_sessions::SessionManagerLayer<tower_sessions::MemoryStore> {
    let session_store = tower_sessions::MemoryStore::default();
    tower_sessions::SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(tower_sessions::cookie::SameSite::None)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::minutes(120),
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

/// Represents an authenticated OpenID Connect (OIDC) user.
///
/// This struct holds the basic user information retrieved from an OIDC provider
/// after a successful login. It contains identifying details such as the user's
/// ID, username, name, and email address.
#[derive(Debug, Default, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE", "name": "John Doe", "username": "JDoe", "email": "john.doe@example.com" }))]
pub struct OidcUser {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
}

/// Extracts an `OidcUser` from the request parts.
///
/// This implementation enables the extraction of the `OidcUser` struct from incoming
/// HTTP request parts using Axum's `FromRequestParts` trait. The user information is
/// retrieved from the OpenID Connect (OIDC) claims, and the required fields (ID, username,
/// name, and email) are extracted from the OIDC token claims.
#[axum::async_trait]
impl<S> FromRequestParts<S> for OidcUser
where
    S: Send + Sync,
{
    type Rejection = <OidcClaims<EmptyAdditionalClaims> as FromRequestParts<
        OidcClaims<EmptyAdditionalClaims>,
    >>::Rejection;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let extractor =
            OidcClaims::<EmptyAdditionalClaims>::from_request_parts(parts, state).await?;
        let id = extractor.subject().to_string();

        let username = extractor
            .preferred_username()
            .ok_or(ExtractorError::Unauthorized)?
            .to_string();
        let name = extractor
            .name()
            .ok_or(ExtractorError::Unauthorized)?
            .get(None)
            .expect("Name is not in correct Langage")
            .to_string();
        let email = extractor
            .email()
            .ok_or(ExtractorError::Unauthorized)?
            .to_string();

        let user = OidcUser {
            id,
            username,
            name,
            email,
        };
        Ok(user)
    }
}
