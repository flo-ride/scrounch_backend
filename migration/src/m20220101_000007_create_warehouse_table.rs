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
                    .table(WarehouseProduct::Table)
                    .if_not_exists()
                    .col(uuid(WarehouseProduct::WarehouseId))
                    .col(uuid(WarehouseProduct::ProductId))
                    .col(decimal_len(WarehouseProduct::Quantity, 10, 2).default(0.0))
                    .col(
                        timestamp_with_time_zone(WarehouseProduct::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(WarehouseProduct::WarehouseId)
                            .col(WarehouseProduct::ProductId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WarehouseProduct::Table, WarehouseProduct::WarehouseId)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WarehouseProduct::Table, WarehouseProduct::ProductId)
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
                    .table(WarehouseRecipe::Table)
                    .if_not_exists()
                    .col(uuid(WarehouseRecipe::WarehouseId))
                    .col(uuid(WarehouseRecipe::RecipeId))
                    .col(integer(WarehouseRecipe::Priority).default(0))
                    .col(
                        timestamp_with_time_zone(WarehouseRecipe::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(WarehouseRecipe::WarehouseId)
                            .col(WarehouseRecipe::RecipeId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WarehouseRecipe::Table, WarehouseRecipe::WarehouseId)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WarehouseRecipe::Table, WarehouseRecipe::RecipeId)
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
            .drop_table(Table::drop().table(WarehouseRecipe::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WarehouseProduct::Table).to_owned())
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
pub enum WarehouseProduct {
    Table,
    WarehouseId,
    ProductId,
    Quantity,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum WarehouseRecipe {
    Table,
    WarehouseId,
    RecipeId,
    Priority,
    CreatedAt,
}
