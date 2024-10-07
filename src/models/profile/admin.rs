//! Application model for representing an admin.
//!
//! This module defines the `Admin` model used by the `scrounch_backend` application
//! to represent admin-related data.

/// Represents an admin within the `scrounch_backend` application.
///
/// This struct is used to encapsulate user information such as their
/// identity, profile, and role within the system.
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE", "name": "John Doe", "username": "JDoe", "email": "john.doe@example.com" }))]
pub struct Admin {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub username: String,
}

impl From<crate::models::profile::user::User> for Admin {
    fn from(value: crate::models::profile::user::User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            username: value.username,
        }
    }
}
