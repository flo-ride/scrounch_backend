use sea_orm::Iterable;
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_configuration_table::{
    Currency, CurrencyVariant, Unit, UnitVariant,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(uuid(Product::Id).primary_key())
                    .col(string_null(Product::Image))
                    .col(string(Product::Name))
                    .col(integer(Product::DisplayOrder).default(0))
                    .col(decimal_len(Product::SellPrice, 10, 2))
                    .col(enumeration(
                        Product::SellPriceCurrency,
                        Currency,
                        CurrencyVariant::iter(),
                    ))
                    .col(small_integer_null(Product::MaxQuantityPerCommand))
                    .col(boolean(Product::Purchasable).default(true))
                    .col(enumeration(Product::Unit, Unit, UnitVariant::iter()))
                    .col(boolean(Product::Disabled).default(false))
                    .col(
                        timestamp_with_time_zone(Product::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(string_null(Product::SmaCode).unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Product {
    Table,
    Id,

    Image,
    // TODO: Gallery Images
    Name, // TODO: Change to i18n
    // TODO: Description / Details (i18n)
    // TODO: Category
    // TODO: Sub Category
    // TODO: Tags
    DisplayOrder,

    SellPrice,
    SellPriceCurrency,
    // TODO: BuyPrice,
    // TODO: BuyPriceCurrency,
    MaxQuantityPerCommand,
    Purchasable,
    Unit,

    Disabled,

    CreatedAt,

    // SMA
    SmaCode,
    // Inventree
    // TODO: InventreeCode,
    // Stripe
    // TODO: StripeCode,
}
