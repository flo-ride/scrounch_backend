//! Application model for representing an OpenID Connect (OIDC) user.
//!
//! This module defines the `OidcUser` struct, which holds user data
//! obtained from an OpenID Connect (OIDC) provider. It is used within the
//! application to manage user authentication and identity through OIDC.

/// Represents an authenticated OpenID Connect (OIDC) user.
///
/// This struct holds the basic user information retrieved from an OIDC provider
/// after a successful login. It contains identifying details such as the user's
/// ID, username, name, and email address.
#[derive(Debug, Default, Clone, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE", "name": "John Doe", "username": "JDoe", "email": "john.doe@example.com" }))]
pub struct OidcUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
}
