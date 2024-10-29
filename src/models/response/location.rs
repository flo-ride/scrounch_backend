use chrono::Utc;

use crate::models::r#enum::location::LocationCategory;

#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct LocationResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub category: Option<LocationCategory>,
    pub creation_time: chrono::DateTime<Utc>,
    pub disabled: bool,
}

impl From<entity::location::Model> for LocationResponse {
    fn from(value: entity::location::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            category: value.category.map(Into::into),
            creation_time: value.creation_time.into(),
            disabled: value.disabled,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct LocationListResponse {
    pub total_page: u64,
    pub current_page: u64,
    pub locations: Vec<LocationResponse>,
}
