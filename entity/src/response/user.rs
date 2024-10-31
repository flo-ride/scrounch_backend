//! This module contains response structures for user-related API responses.
//! It defines the format of data returned to clients regarding users.

/// Represents a response containing user information returned by the API.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE",
    "name": "John Doe",
    "username": "JDoe",
    "email": "john.doe@example.com",
    "is_admin": false,
    "creation_time": "2024-10-09T17:55:30.795279Z",
    "last_access_time": "2024-10-09T17:55:30.795279Z"
}))]
pub struct UserResponse {
    /// Unique identifier for the user.
    pub id: uuid::Uuid,

    /// The email address of the user.
    pub email: String,

    /// The full name of the user.
    pub name: String,

    /// The username chosen by the user.
    pub username: String,

    /// Indicates whether the user has admin privileges.
    pub is_admin: bool,

    /// Indicates whether the user is banned from the application.
    pub is_banned: bool,

    /// The timestamp of when the user was created.
    pub creation_time: chrono::DateTime<chrono::Utc>,

    /// The timestamp of the user's last access.
    pub last_access_time: chrono::DateTime<chrono::Utc>,
}

/// Converts a `Model` from the user module to a `UserResponse`.
impl From<crate::models::user::Model> for UserResponse {
    fn from(value: crate::models::user::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            username: value.username,
            email: value.email,
            is_admin: value.is_admin,
            is_banned: value.is_banned,
            last_access_time: value.last_access_time.into(),
            creation_time: value.creation_time.into(),
        }
    }
}

/// Represents a response containing a list of users returned by the API.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!(
    {
        "total_page": 1, 
        "current_page": 0, 
        "users": [{ 
            "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE", 
            "name": "John Doe", 
            "username": "JDoe", 
            "email": "john.doe@example.com", 
            "is_admin": false, 
            "creation_time": "2024-10-09T17:55:30.795279Z", 
            "last_access_time": "2024-10-09T17:55:30.795279Z" 
        },
        {
            "id": "a1B2c3D4e5F6g7H8i9J0kL1M2N3o4P5Q6R7s8t9U0",
            "name": "Alice Smith",
            "username": "ASmith",
            "email": "alice.smith@example.com",
            "is_admin": false,
            "creation_time": "2024-09-15T12:30:45.123456Z",
            "last_access_time": "2024-10-10T09:15:20.654321Z"
        },
        {
            "id": "Z9Y8x7W6v5U4t3S2r1Q0pO9n8M7l6K5j4I3h2G1f0",
            "name": "Bob Johnson",
            "username": "BJohnson",
            "email": "bob.johnson@example.com",
            "is_admin": true,
            "creation_time": "2024-08-20T14:45:10.987654Z",
            "last_access_time": "2024-10-09T17:55:30.795279Z"
        }]
    }
))]
pub struct UserListResponse {
    /// The total number of pages available for user results.
    pub total_page: u64,

    /// The current page number being viewed.
    pub current_page: u64,

    /// A list of user responses containing user details.
    pub users: Vec<UserResponse>,
}
