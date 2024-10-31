use sea_orm::{
    ActiveValue::{NotSet, Set},
    IntoActiveModel,
};

use crate::models::user::ActiveModel;

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditUserRequest {
    pub is_admin: Option<bool>,
    pub is_banned: Option<bool>,
}

impl IntoActiveModel<ActiveModel> for EditUserRequest {
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
