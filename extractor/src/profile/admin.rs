//! Module for extracting and handling admin profiles in the `scrounch_backend` application.
//!
//! This module defines the `Admin` struct and associated extraction logic for
//! administrative users, as well as error handling through the `AdminExtractorError` enum.
//! It provides the functionality to ensure only authorized users with admin
//! privileges are processed and included in the API responses.

use axum::{extract::FromRequestParts, http::StatusCode};
use service::Connection;

use super::user::{User, UserExtractorError};

/// Represents an admin within the `scrounch_backend` application.
///
/// This struct encapsulates essential admin details such as identity and
/// profile information, specifically for users with administrative privileges.
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE",
    "name": "John Doe",
    "username": "JDoe",
    "email": "john.doe@example.com"
}))]
pub struct Admin {
    /// Unique identifier for the admin.
    pub id: uuid::Uuid,

    /// Optional email address of the admin.
    pub email: Option<String>,

    /// Optional full name of the admin.
    pub name: Option<String>,

    /// Optional username of the admin.
    pub username: Option<String>,
}

impl From<User> for Admin {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            username: value.username,
        }
    }
}

/// Represents potential errors encountered during the extraction of an admin.
///
/// This enum defines errors specific to the extraction and validation of
/// administrative users, including cases where the user is not an admin.
pub enum AdminExtractorError {
    /// Occurs when an error is encountered in the user extraction process.
    UserExtractorError(UserExtractorError),

    /// Occurs when the extracted user does not have admin privileges.
    UserIsNotAdmin(User),
}

impl std::fmt::Display for AdminExtractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserExtractorError(err) => {
                write!(f, "UserExtractorError: {err}")
            }
            Self::UserIsNotAdmin(user) => {
                write!(f, "{user} is not an Admin")
            }
        }
    }
}

impl axum::response::IntoResponse for AdminExtractorError {
    fn into_response(self) -> axum::response::Response {
        log::warn!("{self}");
        match self {
            Self::UserExtractorError(err) => err.into_response(),
            Self::UserIsNotAdmin(_user) => {
                (StatusCode::FORBIDDEN, "You're not an Admin").into_response()
            }
        }
        .into_response()
    }
}

impl std::fmt::Display for Admin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "Admin {} \"{name}\"", self.id)
        } else if let Some(email) = &self.email {
            write!(f, "Admin {} \"{email}\"", self.id)
        } else if let Some(username) = &self.username {
            write!(f, "Admin {} \"{username}\"", self.id)
        } else {
            write!(f, "Admin {}", self.id)
        }
    }
}

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
    type Rejection = AdminExtractorError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let user = User::from_request_parts(parts, state)
            .await
            .map_err(Self::Rejection::UserExtractorError)?;

        let user_is_not_admin = !user.is_admin;
        if user_is_not_admin {
            return Err(Self::Rejection::UserIsNotAdmin(user));
        }

        Ok(user.into())
    }
}
