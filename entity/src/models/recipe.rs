//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the `recipe` table in the database.
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
#[sea_orm(table_name = "recipe")]
pub struct Model {
    /// Primary key of the `recipe` table. Does not auto-increment.
    #[sea_orm(primary_key, auto_increment = false, filter_single)]
    pub id: Uuid,
    /// Foreign key referencing the `product` table.
    #[sea_orm(filter_single)]
    pub result_product_id: Uuid,
    /// Optional name of the recipe.
    pub name: Option<String>,
    /// Indicates whether the recipe is disabled.
    #[sea_orm(filter_single)]
    pub disabled: bool,
    /// Timestamp for when the recipe was created.
    #[sea_orm(filter_override = "chrono::DateTime<chrono::Utc>", filter_plus_order)]
    pub created_at: DateTimeWithTimeZone,
}

/// Defines relationships between `recipe` and other entities.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship: A `recipe` belongs to a `product`.
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ResultProductId",
        to = "super::product::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Product,
    /// Relationship: A `recipe` has many `recipe_ingredients`.
    #[sea_orm(has_many = "super::recipe_ingredients::Entity")]
    RecipeIngredients,
}

impl Related<super::recipe_ingredients::Entity> for Entity {
    /// Defines the direct relationship between `recipe` and `recipe_ingredients`.
    fn to() -> RelationDef {
        Relation::RecipeIngredients.def()
    }
}

impl Related<super::product::Entity> for Entity {
    /// Defines the direct relationship between `recipe` and `product`.
    fn to() -> RelationDef {
        super::recipe_ingredients::Relation::Product.def()
    }
    /// Specifies the intermediary relationship through `recipe_ingredients`.
    fn via() -> Option<RelationDef> {
        Some(super::recipe_ingredients::Relation::Recipe.def().rev())
    }
}

/// Enables customization of the `ActiveModel` for the `recipe` table.
impl ActiveModelBehavior for ActiveModel {}
