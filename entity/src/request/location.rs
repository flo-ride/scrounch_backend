//! Defines structures and conversions related to location requests,
//! including validation and mapping for creating new location entries
//! in the database. Supports validation errors and enum mapping for categories.

use crate::models::{
    location::ActiveModel, sea_orm_active_enums::LocationCategory as ModelLocationCategory,
};
use sea_orm::ActiveValue::{NotSet, Set};

/// The maximum allowed length for a location name.
/// Ensures name fields are concise and conform to database constraints.
pub const LOCATION_NAME_MAX_LENGTH: usize = 32;

/// Represents possible errors encountered in location requests, such as validation issues.
#[derive(Debug, PartialEq, Clone)]
pub enum LocationRequestError {
    /// Occurs when a location name is left empty.
    NameCannotBeEmpty,
    /// Occurs when a location name exceeds the maximum permitted length.
    NameCannotBeLongerThan(String, usize),
}
impl std::error::Error for LocationRequestError {}

impl std::fmt::Display for LocationRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationRequestError::NameCannotBeEmpty => write!(f, "Name Cannot be Empty"),
            LocationRequestError::NameCannotBeLongerThan(name, max) => {
                write!(f, "Name \"{name}\" is longer than {max} characters")
            }
        }
    }
}

/// Enum representing categories of locations, such as dispensers or rooms.
/// This type is used for deserializing request payloads.
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum LocationCategoryRequest {
    /// Represents a dispenser location type.
    Dispenser,
    /// Represents a room location type.
    Room,
}

impl From<LocationCategoryRequest> for ModelLocationCategory {
    fn from(value: LocationCategoryRequest) -> Self {
        match value {
            LocationCategoryRequest::Dispenser => Self::Dispenser,
            LocationCategoryRequest::Room => Self::Room,
        }
    }
}

/// Represents a request to create a new location, including necessary
/// validation for name length and optional category.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewLocationRequest {
    /// The name of the location, subject to length validation.
    pub name: String,
    /// The category of the location, which may be optional.
    pub category: Option<LocationCategoryRequest>,
}

impl TryFrom<NewLocationRequest> for ActiveModel {
    type Error = LocationRequestError;

    /// Converts a `NewLocationRequest` into an `ActiveModel`, performing validation on fields.
    /// Validates `name` to ensure it is not empty and does not exceed the max length.
    fn try_from(value: NewLocationRequest) -> Result<Self, Self::Error> {
        Ok(ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            name: {
                let name = value.name;
                if name.is_empty() {
                    return Err(Self::Error::NameCannotBeEmpty);
                }
                if name.len() > LOCATION_NAME_MAX_LENGTH {
                    return Err(Self::Error::NameCannotBeLongerThan(
                        name,
                        LOCATION_NAME_MAX_LENGTH,
                    ));
                }
                Set(name)
            },
            category: match value.category {
                Some(category) => Set(Some(category.into())),
                None => NotSet,
            },
            disabled: Set(false),
            created_at: Set(chrono::offset::Local::now().into()),
        })
    }
}

/// Represents a request to editing an existing location, including necessary
/// validation for name length and optional category.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditLocationRequest {
    /// The name of the location, subject to length validation.
    pub name: Option<String>,
    /// The category of the location, which may be optional.
    pub category: Option<Option<LocationCategoryRequest>>,
    /// Optional field to disable or enable the product.
    pub disabled: Option<bool>,
}

impl TryFrom<EditLocationRequest> for ActiveModel {
    type Error = LocationRequestError;

    /// Converts a `EditLocationRequest` into an `ActiveModel`, performing validation on fields.
    fn try_from(value: EditLocationRequest) -> Result<Self, Self::Error> {
        Ok(ActiveModel {
            id: NotSet,
            name: match value.name {
                Some(name) => {
                    if name.is_empty() {
                        return Err(Self::Error::NameCannotBeEmpty);
                    }
                    if name.len() > LOCATION_NAME_MAX_LENGTH {
                        return Err(Self::Error::NameCannotBeLongerThan(
                            name,
                            LOCATION_NAME_MAX_LENGTH,
                        ));
                    }
                    Set(name)
                }
                None => NotSet,
            },
            category: match value.category {
                Some(category_opt) => match category_opt {
                    Some(category) => Set(Some(category.into())),
                    None => Set(None),
                },
                None => NotSet,
            },
            disabled: match value.disabled {
                Some(disabled) => Set(disabled),
                None => NotSet,
            },
            created_at: NotSet,
        })
    }
}
