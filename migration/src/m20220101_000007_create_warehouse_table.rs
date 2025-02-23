use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20220101_000003_create_product_table::Product, m20220101_000006_create_recipe_table::Recipe,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Warehouse::Table)
                    .if_not_exists()
                    .col(uuid(Warehouse::Id).primary_key())
                    .col(string(Warehouse::Name))
                    .col(boolean(Warehouse::Disabled).default(false))
                    .col(
                        timestamp_with_time_zone(Warehouse::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WarehouseProducts::Table)
                    .if_not_exists()
                    .col(uuid(WarehouseProducts::WarehouseId))
                    .col(uuid(WarehouseProducts::ProductId))
                    .col(decimal_len(WarehouseProducts::Quantity, 10, 2).default(0.0))
                    .col(
                        timestamp_with_time_zone(WarehouseProducts::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(WarehouseProducts::WarehouseId)
                            .col(WarehouseProducts::ProductId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WarehouseProducts::Table, WarehouseProducts::WarehouseId)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WarehouseProducts::Table, WarehouseProducts::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WarehouseRecipes::Table)
                    .if_not_exists()
                    .col(uuid(WarehouseRecipes::WarehouseId))
                    .col(uuid(WarehouseRecipes::RecipeId))
                    .col(integer(WarehouseRecipes::Priority).default(0))
                    .col(
                        timestamp_with_time_zone(WarehouseRecipes::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(WarehouseRecipes::WarehouseId)
                            .col(WarehouseRecipes::RecipeId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WarehouseRecipes::Table, WarehouseRecipes::WarehouseId)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WarehouseRecipes::Table, WarehouseRecipes::RecipeId)
                            .to(Recipe::Table, Recipe::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WarehouseRecipes::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WarehouseProducts::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Warehouse::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Warehouse {
    Table,
    Id,
    Name,
    Disabled,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum WarehouseProducts {
    Table,
    WarehouseId,
    ProductId,
    Quantity,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum WarehouseRecipes {
    Table,
    WarehouseId,
    RecipeId,
    Priority,
    CreatedAt,
}
