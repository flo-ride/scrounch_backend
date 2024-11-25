//! Defines the `Product` entity, representing a product in the database schema.
//!
//! This entity is managed by SeaORM and includes fields that correspond to
//! columns in the `product` table.
//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15.

use super::sea_orm_active_enums::{Currency, Unit};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the `product` entity in the database, encompassing details about
/// the products available for sale, including pricing, quantities, and images.
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    proc::DeriveToFilterQuery,
)]
#[sea_orm(table_name = "product")]
pub struct Model {
    /// Unique identifier for the product. Primary key, non-auto-incrementing.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// Optional URL or path to the product image.
    pub image: Option<String>,

    /// Name of the product, required for identification.
    pub name: String,

    /// Display Order of the product inside of lists, 0 is last + default
    #[sea_orm(filter_plus_order)]
    pub display_order: i32,

    /// Selling Price of the product, stored as a decimal with up to 10 digits
    /// and 2 decimal places.
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub sell_price: Decimal,

    /// Selling Price Currency for the product,
    #[sea_orm(filter_override = "crate::request::r#enum::CurrencyRequest")]
    pub sell_price_currency: Currency,

    /// If the product is purchasable or if it's just an ingredients
    #[sea_orm(filter_single)]
    pub purchasable: bool,

    /// Represent the unit type of Product, if it's a liquid -> Liter, etc..., the default is Unit
    #[sea_orm(filter_override = "crate::request::r#enum::UnitRequest")]
    pub unit: Unit,

    /// Optional maximum quantity that can be ordered in a single command.
    pub max_quantity_per_command: Option<i16>,

    /// Indicates if the product is currently disabled for sale.
    #[sea_orm(filter_single)]
    pub disabled: bool,

    /// Timestamp indicating when the product was created in the system.
    #[sea_orm(filter_override = "chrono::DateTime<chrono::Utc>", filter_plus_order)]
    pub created_at: DateTimeWithTimeZone,

    /// Optional unique code for the product used by the Sma system.
    #[sea_orm(unique)]
    pub sma_code: Option<String>,

    /// Optional unique code for parts product used by Inventree (IPN) .
    #[sea_orm(unique)]
    pub inventree_code: Option<String>,
}

/// Enum representing the relations of the `Product` entity.
///
/// Currently, there are no defined relations for the `Product` entity,
/// but this enum can be expanded in the future if needed.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
