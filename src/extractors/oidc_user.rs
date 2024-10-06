//! OpenID Connect (OIDC) user extractor for the `scrounch_backend` application.
//!
//! This module defines the `OidcUser` extractor, which is responsible for extracting
//! authenticated user information from the OpenID Connect (OIDC) claims. The extracted
//! user data is used to handle authorization and personalized responses.

use crate::{error::AppError, models::oidc_user::OidcUser};
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
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let extractor = OidcClaims::<EmptyAdditionalClaims>::from_request_parts(parts, state).await;

        match extractor {
            Ok(extractor) => {
                let id = extractor.subject().to_string();
                let uuid = uuid::Uuid::try_parse(&id).map_err(|e| {
                    Self::Rejection::Unknow(format!("Could not Serialise given id: \"{id}\" - {e}"))
                })?;

                let username = extractor
                    .preferred_username()
                    .ok_or(Self::Rejection::MissingOption(
                        "Oidc Extractor is missing username".to_string(),
                    ))?
                    .to_string();
                let name = extractor
                    .name()
                    .ok_or(Self::Rejection::MissingOption(
                        "Oidc Extractor is missing name".to_string(),
                    ))?
                    .get(None)
                    .ok_or(AppError::Unknow(
                        "Name is not in correct Langage".to_string(),
                    ))?
                    .to_string();
                let email = extractor
                    .email()
                    .ok_or(AppError::MissingOption(
                        "Oidc Extractor is missing email".to_string(),
                    ))?
                    .to_string();

                let user = OidcUser {
                    id: uuid,
                    username,
                    name,
                    email,
                };
                Ok(user)
            }
            Err(error) => match error {
                ExtractorError::Unauthorized => Err(Self::Rejection::Forbidden),
                _ => Err(Self::Rejection::OidcError(error)),
            },
        }
    }
}
