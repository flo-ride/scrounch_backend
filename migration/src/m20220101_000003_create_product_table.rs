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
                    .col(
                        decimal_len_null(Product::SellPrice, 10, 2)
                            .default(sea_orm::prelude::Decimal::new(1, 2)),
                    )
                    .col(
                        enumeration_null(
                            Product::SellPriceCurrency,
                            Currency,
                            CurrencyVariant::iter(),
                        )
                        .default(CurrencyVariant::Euro.into_iden().to_string()),
                    )
                    .col(small_integer_null(Product::MaxQuantityPerCommand))
                    .col(
                        enumeration(Product::Unit, Unit, UnitVariant::iter())
                            .default(UnitVariant::Unit.into_iden().to_string()),
                    )
                    .col(boolean(Product::Purchasable).default(true))
                    .col(boolean(Product::Hidden).default(false))
                    .col(boolean(Product::Disabled).default(false))
                    .col(
                        timestamp_with_time_zone(Product::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(string_null(Product::SmaCode).unique_key())
                    .col(string_null(Product::InventreeCode).unique_key())
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
    Unit,

    Purchasable,
    Hidden,
    Disabled,

    CreatedAt,

    // SMA
    SmaCode,
    // Inventree
    InventreeCode,
    // Stripe
    // TODO: StripeCode,
}
