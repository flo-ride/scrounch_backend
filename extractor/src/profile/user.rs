//! Module for handling user profile extraction in the `scrounch_backend` application.
//!
//! This module defines structures and functions related to user profiles, including
//! user data extraction, serialization, and transformations for API responses.
//! It includes key types such as the `User` struct, which represents core user
//! information within the application, and associated error handling.

use axum::{
    extract::{FromRef, FromRequestParts},
    http::StatusCode,
};
use entity::response::user::UserResponse;
use service::{Connection, sea_orm::DbErr};

use super::oidc_user::{OidcUser, OidcUserExtractorError};

/// Represents a user within the `scrounch_backend` application.
///
/// This struct encapsulates core user details such as identity, profile, and
/// access level within the system, including administrative and banned statuses.
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE",
    "name": "John Doe",
    "username": "JDoe",
    "email": "john.doe@example.com",
    "is_admin": false
}))]
pub struct User {
    /// Unique identifier for the user.
    pub id: uuid::Uuid,

    /// Optional email address of the user.
    pub email: Option<String>,

    /// Optional full name of the user.
    pub name: Option<String>,

    /// Optional username of the user.
    pub username: Option<String>,

    /// Indicates if the user has administrative privileges.
    pub is_admin: bool,

    /// Indicates if the user is currently banned from the system.
    pub is_banned: bool,

    /// Timestamp indicating when the user was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Timestamp of the user's last access within the system.
    pub last_access_at: chrono::DateTime<chrono::Utc>,
}

impl From<entity::models::user::Model> for User {
    fn from(value: entity::models::user::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            username: value.username,
            is_admin: value.is_admin,
            is_banned: value.is_banned,
            last_access_at: value.last_access_at.into(),
            created_at: value.created_at.into(),
        }
    }
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            username: value.username,
            email: value.email,
            is_admin: value.is_admin,
            is_banned: value.is_banned,
            last_access_at: value.last_access_at,
            created_at: value.created_at,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "User {} \"{name}\"", self.id)
        } else if let Some(email) = &self.email {
            write!(f, "User {} \"{email}\"", self.id)
        } else if let Some(username) = &self.username {
            write!(f, "User {} \"{username}\"", self.id)
        } else {
            write!(f, "User {}", self.id)
        }
    }
}

/// Errors that may occur while extracting a user from the system.
///
/// This enum encompasses various error types related to user extraction,
/// including issues with OIDC extraction, database retrieval, and user status.
pub enum UserExtractorError {
    /// Error when extracting an `OidcUser`, typically due to authentication or serialization issues.
    OidcUserExtractorError(OidcUserExtractorError),

    /// Error indicating that no user was found with the specified UUID.
    DidntFindUser(uuid::Uuid),

    /// Database-related error encountered while attempting to retrieve user information.
    DatabaseError(DbErr),

    /// Error indicating that the user with the specified UUID is banned.
    UserIsBanned(uuid::Uuid),
}

impl std::fmt::Display for UserExtractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OidcUserExtractorError(err) => {
                write!(f, "OidcUserExtractorError: {err}")
            }
            Self::DidntFindUser(id) => {
                write!(f, "We can't find user with id \"{id}\" in database")
            }
            Self::UserIsBanned(id) => {
                write!(f, "User \"{id}\" is banned")
            }
            Self::DatabaseError(db) => {
                write!(f, "An Database error happened: {db}")
            }
        }
    }
}

impl axum::response::IntoResponse for UserExtractorError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::OidcUserExtractorError(err) => err.into_response(),
            Self::DidntFindUser(_id) => {
                (StatusCode::FORBIDDEN, "Sorry but don't know you").into_response()
            }
            Self::UserIsBanned(_id) => (
                StatusCode::FORBIDDEN,
                "Nice try but the ban hammer has talked",
            )
                .into_response(),
            Self::DatabaseError(err) => {
                log::warn!("{err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Sorry but we face a problem, please contact us if this remain",
                )
                    .into_response()
            }
        }
    }
}

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
    type Rejection = UserExtractorError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let conn = Connection::from_ref(state);
        let oidc_user = OidcUser::from_request_parts(parts, state)
            .await
            .map_err(Self::Rejection::OidcUserExtractorError)?;
        let id = oidc_user.id;

        let user = service::Query::find_user_by_id(&conn, id)
            .await
            .map_err(Self::Rejection::DatabaseError)?
            .ok_or(Self::Rejection::DidntFindUser(id))?; // This sould never happen

        match user.is_banned {
            true => Err(Self::Rejection::UserIsBanned(id)),
            false => Ok(user.into()),
        }
    }
}
