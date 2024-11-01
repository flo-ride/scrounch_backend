//! `SeaORM` entity representing the `refill` table. This entity defines the
//! structure of refills, including unique identifiers, timestamps, amounts
//! in euros and "epicoin", and an active state marker.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Main struct representing a refill entry in the database.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "refill")] // Associates this struct with the "refill" table.
pub struct Model {
    /// Unique identifier for the refill, not auto-incremented.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// Optional name associated with the refill entry.
    pub name: Option<String>,

    /// Timestamp indicating when the refill was created.
    pub creation_time: DateTimeWithTimeZone,

    /// Amount in euros, with a precision of 10 digits and 2 decimal places.
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount_in_euro: Decimal,

    /// Amount in "epicoin", with a precision of 10 digits and no decimal places.
    #[sea_orm(column_type = "Decimal(Some((10, 0)))")]
    pub amount_in_epicoin: Decimal,

    /// Boolean flag indicating whether the refill entry is disabled.
    pub disabled: bool,
}

/// Enum representing relationships for the `refill` entity.
/// Currently, there are no relationships defined for this entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// Custom behavior for the `ActiveModel` of the `refill` entity.
/// By default, SeaORM provides basic behavior, so no custom behavior is defined here.
impl ActiveModelBehavior for ActiveModel {}
