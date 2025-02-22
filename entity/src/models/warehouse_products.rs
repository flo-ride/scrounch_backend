//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Struct representing the `warehouse_products` table model in SeaORM.
/// Contains fields corresponding to the columns in the `warehouse_products` table.
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    proc::DeriveToFilterQuery,
    proc::DeriveToSortQuery,
)]
#[sea_orm(table_name = "warehouse_products")]

pub struct Model {
    /// The unique identifier for the warehouse.
    #[sea_orm(primary_key, auto_increment = false)]
    pub warehouse_id: Uuid,
    /// The unique identifier for the product.
    #[sea_orm(primary_key, auto_increment = false)]
    pub product_id: Uuid,

    /// The quantity of this product in this warehouse
    pub quantity: i32,

    /// Timestamp for when the link was created.
    #[sea_orm(filter_override = "chrono::DateTime<chrono::Utc>", filter_plus_order)]
    pub created_at: DateTimeWithTimeZone,
}

/// Enum representing the relationships of the `warehouse_products` entity in SeaORM.
/// Defines how `warehouse` is related to other entities, enabling query joins.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Represent the product of this many/many relation
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ProductId",
        to = "super::product::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Product,

    /// Represent the warehouse of this many/many relation
    #[sea_orm(
        belongs_to = "super::warehouse::Entity",
        from = "Column::WarehouseId",
        to = "super::warehouse::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Warehouse,
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::warehouse::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Warehouse.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
