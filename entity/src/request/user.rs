//! Defines the `EditUserRequest` struct, which represents an editable request
//! to update specific user attributes. This struct allows partial updates
//! to the `User` entity in the database, focusing on attributes such as
//! `is_admin` and `is_banned`.

use sea_orm::{
    ActiveValue::{NotSet, Set},
    IntoActiveModel,
};

use crate::models::user::ActiveModel;

/// Represents an update request for user-specific fields, allowing modification
/// of key boolean attributes such as `is_admin` and `is_banned`.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditUserRequest {
    /// Indicates whether the user has admin privileges.
    pub is_admin: Option<bool>,
    /// Indicates whether the user is banned from the system.
    pub is_banned: Option<bool>,
}

impl IntoActiveModel<ActiveModel> for EditUserRequest {
    /// Converts an `EditUserRequest` into an `ActiveModel` for database updates.
    /// Fields set to `Some` are updated, while fields with `None` remain unchanged.
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            is_admin: match self.is_admin {
                Some(is_admin) => Set(is_admin),
                None => NotSet,
            },
            is_banned: match self.is_banned {
                Some(is_banned) => Set(is_banned),
                None => NotSet,
            },
            ..Default::default()
        }
    }
}
