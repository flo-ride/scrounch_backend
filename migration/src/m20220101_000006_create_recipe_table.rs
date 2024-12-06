use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000003_create_product_table::Product;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(uuid(Recipe::Id).primary_key())
                    .col(uuid(Recipe::ResultProductId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Recipe::Table, Recipe::ResultProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string_null(Recipe::Name))
                    .col(boolean(Recipe::Disabled).default(false))
                    .col(
                        timestamp_with_time_zone(Recipe::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RecipeIngredients::Table)
                    .if_not_exists()
                    .col(uuid(RecipeIngredients::RecipeId))
                    .col(uuid(RecipeIngredients::IngredientId))
                    .primary_key(
                        Index::create()
                            .col(RecipeIngredients::RecipeId)
                            .col(RecipeIngredients::IngredientId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeIngredients::Table, RecipeIngredients::RecipeId)
                            .to(Recipe::Table, Recipe::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeIngredients::Table, RecipeIngredients::IngredientId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(decimal_len(RecipeIngredients::Quantity, 10, 2))
                    .col(boolean(RecipeIngredients::Disabled).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecipeIngredients::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Recipe::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Recipe {
    Table,
    Id,
    Name,
    ResultProductId,
    CreatedAt,
    Disabled,
}

#[derive(DeriveIden)]
pub enum RecipeIngredients {
    Table,
    RecipeId,
    IngredientId,
    Quantity,
    Disabled,
}
