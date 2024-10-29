use crate::models::r#enum::location::LocationCategory;

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewLocation {
    pub name: String,
    pub category: Option<LocationCategory>,
}

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditLocation {
    pub name: Option<String>,
    pub category: Option<LocationCategory>,
    pub disabled: Option<bool>,
}
