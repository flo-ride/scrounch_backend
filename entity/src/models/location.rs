//! Defines the `Location` entity, representing a location in the database schema.
//!
//! This entity is managed by SeaORM and includes fields that correspond to
//! columns in the `location` table.
//! `SeaORM` Entity. Generated by sea-orm-codegen.

use super::sea_orm_active_enums::LocationCategory;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a location in the database schema.
///
/// This struct is managed by SeaORM and includes fields that correspond to
/// columns in the `location` table. It includes a unique identifier (`id`),
/// a name, the creation time of the location, an optional category,
/// and a flag indicating whether the location is disabled
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "location")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    /// Unique identifier for the location.
    /// This field serves as the primary key and is not auto-incremented.
    pub id: Uuid,

    #[sea_orm(unique)]
    /// Name of the location.
    /// This field must be unique within the database.
    pub name: String,

    /// Timestamp of when the location was created.
    /// This field stores the creation time with timezone information.
    pub created_at: DateTimeWithTimeZone,

    /// Optional category for the location.
    /// This field can be used to classify the location into different categories.
    pub category: Option<LocationCategory>,

    /// Flag indicating if the location is disabled.
    /// This field is a boolean that signifies whether the location is active or not.
    pub disabled: bool,
}

/// Enum representing the relations of the `Location` entity.
///
/// Currently, there are no defined relations for the `Location` entity,
/// but this enum can be expanded in the future if needed.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
