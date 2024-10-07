//! Admin extractors for the `scrounch_backend` application.
//!
//! This module provides implementations for extracting administrative user
//! information from HTTP requests within the `scrounch_backend` application.
//! The extractors ensure that only users with administrative privileges can
//! access certain routes or functionalities, thereby enforcing access control.

use crate::{
    error::AppError,
    models::profile::{admin::Admin, user::User},
};
use axum::extract::FromRequestParts;
use service::Connection;

/// Extractor implementation for retrieving an `Admin` from HTTP request parts.
///
/// This implementation of `FromRequestParts` allows Axum to automatically
/// extract an `Admin` instance from incoming HTTP requests. It ensures that
/// the user is authenticated and has administrative privileges.
#[axum::async_trait]
impl<S> FromRequestParts<S> for Admin
where
    Connection: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let user = User::from_request_parts(parts, state).await?;

        let user_is_not_admin = !user.is_admin;
        if user_is_not_admin {
            return Err(AppError::Forbidden);
        }

        Ok(user.into())
    }
}
