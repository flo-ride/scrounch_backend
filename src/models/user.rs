//! Application model for representing a user.
//!
//! This module defines the `User` model used by the `scrounch_backend` application
//! to represent user-related data.

/// Represents a user within the `scrounch_backend` application.
///
/// This struct is used to encapsulate user information such as their
/// identity, profile, and role within the system.
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE", "name": "John Doe", "username": "JDoe", "email": "john.doe@example.com", "is_admin": false }))]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub username: String,
    pub is_admin: bool,
}

impl From<entity::user::Model> for User {
    fn from(value: entity::user::Model) -> Self {
        Self {
            id: value.id.to_string(),
            email: value.email,
            name: value.name,
            username: value.username,
            is_admin: value.is_admin,
        }
    }
}
