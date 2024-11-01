use sea_orm_migration::{prelude::*, schema::*};

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
                        timestamp_with_time_zone(Refill::CreationTime)
                            .default(Expr::current_timestamp()),
                    )
                    .col(decimal_len(Refill::AmountInEuro, 10, 2))
                    .col(decimal_len(Refill::AmountInEpicoin, 10, 0))
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
enum Refill {
    Table,
    Id,
    Name,
    AmountInEuro,
    AmountInEpicoin,
    CreationTime,
    Disabled,
}
