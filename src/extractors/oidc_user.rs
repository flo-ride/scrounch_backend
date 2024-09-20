//! OpenID Connect (OIDC) user extractor for the `scrounch_backend` application.
//!
//! This module defines the `OidcUser` extractor, which is responsible for extracting
//! authenticated user information from the OpenID Connect (OIDC) claims. The extracted
//! user data is used to handle authorization and personalized responses.

use crate::models::oidc_user::OidcUser;
use axum::extract::FromRequestParts;
use axum_oidc::{error::ExtractorError, EmptyAdditionalClaims, OidcClaims};

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
