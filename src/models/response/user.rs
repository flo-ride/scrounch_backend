use chrono::Utc;

use crate::models::profile::user::User;

#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "id": "l8F0ZoHb5TwYgNvXkJqV7SsP9gQfKzR4UmA1VrCwIxE", "name": "John Doe", "username": "JDoe", "email": "john.doe@example.com", "is_admin": false, "creation_time": "2024-10-09T17:55:30.795279Z", "last_access_time": "2024-10-09T17:55:30.795279Z" }))]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub username: String,
    pub is_admin: bool,
    pub is_banned: bool,
    pub creation_time: chrono::DateTime<Utc>,
    pub last_access_time: chrono::DateTime<Utc>,
}

impl From<entity::models::user::Model> for UserResponse {
    fn from(value: entity::models::user::Model) -> Self {
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

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            username: value.username,
            email: value.email,
            is_admin: value.is_admin,
            is_banned: value.is_banned,
            last_access_time: value.last_access_time,
            creation_time: value.creation_time,
        }
    }
}

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
    pub total_page: u64,
    pub current_page: u64,
    pub users: Vec<UserResponse>,
}
