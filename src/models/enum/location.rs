use entity::models::sea_orm_active_enums::LocationCategory as ModelLocationCategory;

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum LocationCategory {
    Dispenser,
    Room,
}

impl From<ModelLocationCategory> for LocationCategory {
    fn from(value: ModelLocationCategory) -> Self {
        match value {
            ModelLocationCategory::Dispenser => Self::Dispenser,
            ModelLocationCategory::Room => Self::Room,
        }
    }
}

impl From<LocationCategory> for ModelLocationCategory {
    fn from(value: LocationCategory) -> Self {
        match value {
            LocationCategory::Dispenser => Self::Dispenser,
            LocationCategory::Room => Self::Room,
        }
    }
}
