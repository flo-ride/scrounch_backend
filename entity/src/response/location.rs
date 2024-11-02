//! Defines response types for location-related API endpoints, including structures
//! for single locations and paginated lists of locations. The responses provide
//! information on locations, such as unique identifiers, names, categories,
//! creation timestamps, and statuses. The `LocationCategoryResponse` enum maps
//! location categories, while `LocationResponse` and `LocationListResponse`
//! represent individual and paginated location responses respectively.

use crate::models::sea_orm_active_enums::LocationCategory as ModelLocationCategory;

/// Enum representing categories of locations, such as dispensers or rooms.
/// This type is used for serializing request payloads and indicates the
/// type of location being referred to in a response.
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum LocationCategoryResponse {
    /// Represents a dispenser location type.
    Dispenser,
    /// Represents a room location type.
    Room,
}

impl From<ModelLocationCategory> for LocationCategoryResponse {
    fn from(value: ModelLocationCategory) -> Self {
        match value {
            ModelLocationCategory::Dispenser => Self::Dispenser,
            ModelLocationCategory::Room => Self::Room,
        }
    }
}

/// Response structure representing a location entity, including its
/// unique identifier, name, category, creation timestamp, and status.
///
/// This structure is used for API responses where individual location details
/// are required.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct LocationResponse {
    /// Unique identifier of the location.
    pub id: uuid::Uuid,

    /// The name of the location.
    pub name: String,

    /// The category of the location, which could be `Dispenser` or `Room`.
    pub category: Option<LocationCategoryResponse>,

    /// The timestamp indicating when the location was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Indicates whether the location is disabled.
    pub disabled: bool,
}

impl From<crate::models::location::Model> for LocationResponse {
    fn from(value: crate::models::location::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            category: value.category.map(Into::into),
            created_at: value.created_at.into(),
            disabled: value.disabled,
        }
    }
}

/// Response structure representing a paginated list of locations.
///
/// This structure is intended for use in API responses where a paginated
/// list of locations is required.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct LocationListResponse {
    /// The total number of pages available.
    pub total_page: u64,

    /// The current page number in the paginated response.
    pub current_page: u64,

    /// A list of locations on the current page, represented by `LocationResponse`.
    pub locations: Vec<LocationResponse>,
}
