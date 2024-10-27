#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditUser {
    pub is_admin: Option<bool>,
}
