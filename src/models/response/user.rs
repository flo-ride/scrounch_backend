use chrono::Utc;

use crate::models::profile::user::User;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub username: String,
    pub is_admin: bool,
    pub creation_time: chrono::DateTime<Utc>,
    pub last_access_time: chrono::DateTime<Utc>,
}

impl From<entity::user::Model> for UserResponse {
    fn from(value: entity::user::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            username: value.username,
            email: value.email,
            is_admin: value.is_admin,
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
            last_access_time: value.last_access_time,
            creation_time: value.creation_time,
        }
    }
}
