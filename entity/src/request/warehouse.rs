//! # Warehouse Request Models and Error Handling
//! This module defines the structures and associated logic for handling warehouse requests in the system,
//! including both the creation and editing of warehouses.

use crate::{error::impl_bad_request_app_error, models::warehouse::ActiveModel};
use sea_orm::ActiveValue::{NotSet, Set};

/// The maximum allowed length for a warehouse name.
/// This constraint ensures that names remain concise and standardized in the database.
pub const WAREHOUSE_NAME_MAX_LENGTH: usize = 32;

/// Enum representing potential errors in the warehouse request validation process.
#[derive(Debug, PartialEq, Clone, strum_macros::IntoStaticStr)]
pub enum WarehouseRequestError {
    /// Error when the warehouse name is empty.
    NameCannotBeEmpty,
    /// Error when the warehouse name exceeds the allowed maximum length.
    NameCannotBeLongerThan(String, usize),
    /// Error when the parent  doesn't exist
    ParentDoesntExist(uuid::Uuid),
    /// Error when the parent of a given warehouse is self
    ParentCannotBeSelf(uuid::Uuid),
    /// Error when the parents of multiple are circularly connected
    ParentCannotHaveCircularReference(uuid::Uuid, uuid::Uuid, Vec<uuid::Uuid>),
}

impl std::error::Error for WarehouseRequestError {}

/// Formats error messages for the `WarehouseRequestError` enum.
impl std::fmt::Display for WarehouseRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameCannotBeEmpty => write!(f, "Name cannot be empty"),
            Self::NameCannotBeLongerThan(name, max) => {
                write!(f, "Name \"{name}\" is longer than {max} characters")
            }
            Self::ParentDoesntExist(parent) => {
                write!(f, "Parent \"{parent}\" doesn't exist")
            }
            Self::ParentCannotBeSelf(parent) => {
                write!(
                    f,
                    "Parent \"{parent}\" of warehouse \"{parent}\" canno't be self"
                )
            }
            Self::ParentCannotHaveCircularReference(warehouse, parent, circular) => {
                let circular_string = circular.iter().map(ToString::to_string).collect::<Vec<_>>();
                write!(f, "Warehouse \"{warehouse}\" cannot have \"{parent}\" has parent, it would create a circular reference: \"{}\"", circular_string.join("->"))
            }
        }
    }
}
impl_bad_request_app_error!(WarehouseRequestError);

/// Request structure for creating a new warehouse, including validation rules.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewWarehouseRequest {
    /// Name of the warehouse, required and validated for length.
    pub name: String,

    /// Name of the parent warehouse
    pub parent: Option<uuid::Uuid>,
}

/// Converts `NewWarehouseRequest` into `ActiveModel` with validation.
/// Errors are returned if validation fails.
impl TryFrom<NewWarehouseRequest> for ActiveModel {
    type Error = WarehouseRequestError;

    fn try_from(value: NewWarehouseRequest) -> Result<Self, Self::Error> {
        Ok(ActiveModel {
            id: Set(uuid::Uuid::new_v4()), // Automatically generate a new UUID for the ID.
            name: {
                let name = value.name;
                if name.is_empty() {
                    return Err(Self::Error::NameCannotBeEmpty);
                }
                if name.len() > WAREHOUSE_NAME_MAX_LENGTH {
                    return Err(Self::Error::NameCannotBeLongerThan(
                        name,
                        WAREHOUSE_NAME_MAX_LENGTH,
                    ));
                }
                Set(name)
            },
            parent: Set(value.parent),
        })
    }
}

/// Structure representing a request to edit an existing warehouse entry.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditWarehouseRequest {
    /// New name for the Warehouse.
    pub name: Option<String>,

    /// Optional New Parent for the Warehouse
    pub parent: Option<Option<uuid::Uuid>>,
}

/// Converts `EditWarehouseRequest` into `ActiveModel` with validation.
/// Only fields present in the request are updated.
impl TryFrom<EditWarehouseRequest> for ActiveModel {
    type Error = WarehouseRequestError;

    fn try_from(value: EditWarehouseRequest) -> Result<Self, Self::Error> {
        Ok(ActiveModel {
            id: NotSet, // ID is not updated during edits.
            name: match value.name {
                Some(name) => {
                    if name.is_empty() {
                        return Err(Self::Error::NameCannotBeEmpty);
                    }
                    if name.len() > WAREHOUSE_NAME_MAX_LENGTH {
                        return Err(Self::Error::NameCannotBeLongerThan(
                            name,
                            WAREHOUSE_NAME_MAX_LENGTH,
                        ));
                    }
                    Set(name)
                }
                None => NotSet,
            },
            parent: match value.parent {
                Some(parent_opt) => Set(parent_opt),
                None => NotSet,
            },
        })
    }
}
