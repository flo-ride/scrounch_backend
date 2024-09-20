//! User extractors for the `scrounch_backend` application.
//!
//! This module contains implementations for extracting user-related information
//! from HTTP requests within the `scrounch_backend` application. These extractors
//! are used to retrieve user data, typically associated with authentication
//! processes, making it easier to access user information in route handlers.

use crate::{error::AppError, models::oidc_user::OidcUser, models::user::User};
use axum::extract::{FromRef, FromRequestParts};
use service::Connection;

/// Extractor implementation for retrieving a `User` from HTTP request parts.
///
/// This implementation of `FromRequestParts` allows Axum to automatically
/// extract a `User` instance from incoming HTTP requests.
#[axum::async_trait]
impl<S> FromRequestParts<S> for User
where
    Connection: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let conn = Connection::from_ref(state);
        let oidc_user = OidcUser::from_request_parts(parts, state).await?;

        let user = service::Query::find_user_by_id(&conn, oidc_user.id)
            .await?
            .ok_or(AppError::Unknow)?; // This sould never happen

        Ok(user.into())
    }
}
