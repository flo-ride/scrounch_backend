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
                    .table(User::Table)
                    .if_not_exists()
                    .col(uuid(User::Id).primary_key())
                    .col(string_null(User::Email))
                    .col(string_null(User::Name))
                    .col(string_null(User::Username))
                    .col(decimal_len(User::Balance, 10, 2).default(0.0))
                    .col(enumeration(
                        User::BalanceCurrency,
                        Currency,
                        CurrencyVariant::iter(),
                    ))
                    .col(boolean(User::IsAdmin).default(false))
                    .col(boolean(User::IsBanned).default(false))
                    .col(
                        timestamp_with_time_zone(User::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(User::LastAccessAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Username,
    Balance,
    BalanceCurrency,
    IsAdmin,
    IsBanned,
    CreatedAt,
    LastAccessAt,
}
