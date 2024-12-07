//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the `recipe_ingredients` table in the database.
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
#[sea_orm(table_name = "recipe_ingredients")]
pub struct Model {
    /// Primary key: The ID of the associated recipe.
    #[sea_orm(primary_key, auto_increment = false)]
    pub recipe_id: Uuid,
    /// Primary key: The ID of the associated ingredient.
    #[sea_orm(primary_key, auto_increment = false)]
    pub ingredient_id: Uuid,
    /// The quantity of the ingredient used in the recipe.
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub quantity: Decimal,

    /// Indicates if this ingredient is disable
    #[sea_orm(filter_single)]
    pub disabled: bool,
}

/// Defines relationships between `recipe_ingredients` and other entities.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Relationship: A `recipe_ingredient` belongs to a `product` (ingredient).
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::IngredientId",
        to = "super::product::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Product,
    /// Relationship: A `recipe_ingredient` belongs to a `recipe`.
    #[sea_orm(
        belongs_to = "super::recipe::Entity",
        from = "Column::RecipeId",
        to = "super::recipe::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Recipe,
}

impl Related<super::product::Entity> for Entity {
    /// Defines the direct relationship between `recipe_ingredients` and `product`.
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::recipe::Entity> for Entity {
    /// Defines the direct relationship between `recipe_ingredients` and `recipe`.
    fn to() -> RelationDef {
        Relation::Recipe.def()
    }
}

/// Enables customization of the `ActiveModel` for the `recipe_ingredients` table.
impl ActiveModelBehavior for ActiveModel {}