use sea_orm::{DbBackend, EnumIter, Iterable};
use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

use crate::{
    m20220101_000001_create_configuration_table::{Currency, CurrencyVariant},
    m20220101_000002_create_user_table::User,
    m20220101_000005_create_refill_table::Refill,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .create_type(
                        Type::create()
                            .as_enum(TransactionType)
                            .values(TransactionTypeVariant::iter())
                            .to_owned(),
                    )
                    .await?;

                manager
                    .create_type(
                        Type::create()
                            .as_enum(TransactionStatus)
                            .values(TransactionStatusVariant::iter())
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(uuid(Transaction::Id).primary_key())
                    .col(uuid(Transaction::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Transaction::Table, Transaction::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        timestamp_with_time_zone(Transaction::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Transaction::CompletedAt))
                    .col(enumeration(
                        Transaction::Status,
                        TransactionStatus,
                        TransactionStatusVariant::iter(),
                    ))
                    .col(enumeration(
                        Transaction::Type,
                        TransactionType,
                        TransactionTypeVariant::iter(),
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TransactionRefillBalance::Table)
                    .if_not_exists()
                    .col(uuid(TransactionRefillBalance::Id).primary_key())
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                TransactionRefillBalance::Table,
                                TransactionRefillBalance::Id,
                            )
                            .to(Transaction::Table, Transaction::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(uuid_null(TransactionRefillBalance::RefillId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                TransactionRefillBalance::Table,
                                TransactionRefillBalance::RefillId,
                            )
                            .to(Refill::Table, Refill::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(decimal_len(TransactionRefillBalance::Price, 10, 2))
                    .col(enumeration(
                        TransactionRefillBalance::PriceCurrency,
                        Currency,
                        CurrencyVariant::iter(),
                    ))
                    .col(decimal_len(TransactionRefillBalance::Credit, 10, 2))
                    .col(enumeration(
                        TransactionRefillBalance::CreditCurrency,
                        Currency,
                        CurrencyVariant::iter(),
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TransactionPurchaseWithBalance::Table)
                    .if_not_exists()
                    .col(uuid(TransactionPurchaseWithBalance::Id).primary_key())
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                TransactionPurchaseWithBalance::Table,
                                TransactionPurchaseWithBalance::Id,
                            )
                            .to(Transaction::Table, Transaction::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(TransactionRefillBalance::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(TransactionPurchaseWithBalance::Table)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .drop_type(Type::drop().name(TransactionType).to_owned())
                    .await?;
                manager
                    .drop_type(Type::drop().name(TransactionStatus).to_owned())
                    .await?;
            }
        }
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Transaction {
    Table,
    Id,
    UserId,
    Status,
    Type,
    CreatedAt,
    CompletedAt,
}

#[derive(DeriveIden)]
pub struct TransactionType;

#[derive(DeriveIden, EnumIter)]
pub enum TransactionTypeVariant {
    RefillBalance,
    BalanceEdit,
    PurchaseWithBalance,
    PurchaseDirect,
}

#[derive(DeriveIden)]
pub struct TransactionStatus;

#[derive(DeriveIden, EnumIter)]
pub enum TransactionStatusVariant {
    Created,
    Pending,
    Completed,
    Failed,
    Canceled,
}

#[derive(DeriveIden)]
pub enum TransactionRefillBalance {
    Table,
    Id,

    RefillId,

    Price,
    PriceCurrency,

    Credit,
    CreditCurrency,

    StripeTransactionId,
}

#[derive(DeriveIden)]
pub enum TransactionPurchaseWithBalance {
    Table,
    Id,

    PriceTotal,
    PriceTotalCurrency,
}

pub enum TransactionProduct {
    Table,
    Id,
    ProductId,
    Name,
    Price,
    PriceCurrency,
}
