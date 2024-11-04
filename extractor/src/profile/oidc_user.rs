//! This module defines an extractor for handling OpenID Connect (OIDC) user data.
//!
//! The `OidcUser` extractor retrieves and validates OIDC claims from incoming requests,
//! allowing secure access to user identity information based on OIDC standards.
//! It simplifies the integration of authentication mechanisms by providing a structured
//! representation of OIDC user data for further use in profile-related operations.

use axum::http::StatusCode;
use axum_oidc::{error::ExtractorError, EmptyAdditionalClaims, OidcClaims};

/// Represents an OpenID Connect (OIDC) user with essential profile information.
///
/// This structure is used to store and serialize the OIDC user data extracted from
/// identity claims. It provides basic user attributes such as `id`, `username`,
/// `name`, and `email`, which can be utilized in various profile-related
/// operations within the application.
#[derive(Debug, Default, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE",
    "name": "John Doe",
    "username": "JDoe",
    "email": "john.doe@example.com"
}))]
pub struct OidcUser {
    /// Unique identifier for the OIDC user.
    pub id: uuid::Uuid,

    /// Optional username of the user.
    pub username: Option<String>,

    /// Optional full name of the user.
    pub name: Option<String>,

    /// Optional email address of the user.
    pub email: Option<String>,
}

/// Errors that can occur while extracting an `OidcUser`.
///
/// This enum defines the potential error types that may be encountered when
/// handling `OidcUser` extraction, covering issues with ID serialization,
/// authorization failures, and other extraction-specific errors.
pub enum OidcUserExtractorError {
    /// Error when serializing the user ID, with a description and the original UUID error.
    CannotSerializeId(String, uuid::Error),

    /// Error indicating that the user is unauthorized.
    Unauthorized,

    /// General extraction error, wrapping a specific `ExtractorError`.
    ExtractorError(ExtractorError),
}

impl std::fmt::Display for OidcUserExtractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExtractorError(err) => {
                write!(f, "This user faced a problem with OpenID Extraction: {err}")
            }
            Self::Unauthorized => write!(f, "This user isn't able to login using this OpenID user"),
            Self::CannotSerializeId(id, err) => {
                write!(f, "We failed to Serialize this user id: \"{id}\" - {err}")
            }
        }
    }
}

impl axum::response::IntoResponse for OidcUserExtractorError {
    fn into_response(self) -> axum::response::Response {
        log::warn!("{self}");
        match self {
            Self::CannotSerializeId(_id, _err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Sorry but it seems we cannot Serialize your OpenID user id",
            ),
            Self::Unauthorized => (StatusCode::FORBIDDEN, "You cannot use this app"),
            Self::ExtractorError(_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Sorry but we've faced a problem with you",
            ),
        }
        .into_response()
    }
}

/// Extracts an `OidcUser` from the request parts.
///
/// This implementation enables the extraction of the `OidcUser` struct from incoming
/// HTTP request parts using Axum's `FromRequestParts` trait. The user information is
/// retrieved from the OpenID Connect (OIDC) claims, and the required fields (ID, username,
/// name, and email) are extracted from the OIDC token claims.
#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for OidcUser
where
    S: Send + Sync,
{
    type Rejection = OidcUserExtractorError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let extractor = OidcClaims::<EmptyAdditionalClaims>::from_request_parts(parts, state).await;

        match extractor {
            Ok(extractor) => {
                let id = extractor.subject().to_string();
                let uuid = uuid::Uuid::try_parse(&id)
                    .map_err(|e| Self::Rejection::CannotSerializeId(id, e))?;

                let username = extractor.preferred_username().map(|x| x.to_string());

                let name = extractor
                    .name()
                    .map(|x| x.get(None))
                    .and_then(|x| x.map(|x| x.to_string()));

                let email = extractor.email().map(|x| x.to_string());

                let user = OidcUser {
                    id: uuid,
                    username,
                    name,
                    email,
                };
                Ok(user)
            }
            Err(error) => match error {
                ExtractorError::Unauthorized => Err(Self::Rejection::Unauthorized),
                _ => Err(Self::Rejection::ExtractorError(error)),
            },
        }
    }
}
