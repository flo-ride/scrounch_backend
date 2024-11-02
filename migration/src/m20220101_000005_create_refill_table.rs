use sea_orm::Iterable;
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_configuration_table::{Currency, CurrencyVariant};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Refill::Table)
                    .if_not_exists()
                    .col(uuid(Refill::Id).primary_key())
                    .col(string_null(Refill::Name))
                    .col(
                        timestamp_with_time_zone(Refill::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(decimal_len(Refill::Price, 10, 2))
                    .col(enumeration(
                        Refill::PriceCurrency,
                        Currency,
                        CurrencyVariant::iter(),
                    ))
                    .col(decimal_len(Refill::Credit, 10, 2))
                    .col(enumeration(
                        Refill::CreditCurrency,
                        Currency,
                        CurrencyVariant::iter(),
                    ))
                    .col(boolean(Refill::Disabled).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Refill::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Refill {
    Table,
    Id,
    Name,
    Price,
    PriceCurrency,
    Credit,
    CreditCurrency,
    CreatedAt,
    Disabled,
}
