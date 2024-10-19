use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(uuid(Product::Id).primary_key())
                    .col(string_null(Product::Image))
                    .col(string(Product::Name))
                    .col(decimal(Product::Price)) // If you ask: why not money ? here is the answer: https://wiki.postgresql.org/wiki/Don't_Do_This#Don.27t_use_money
                    .col(small_unsigned(Product::Quantity))
                    .col(small_unsigned_null(Product::MaxQuantityPerCommand))
                    .col(boolean(Product::Disabled).default(false))
                    .col(
                        timestamp_with_time_zone(Product::CreationTime)
                            .default(Expr::current_timestamp()),
                    )
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
enum Product {
    Table,
    Id,
    Image,
    Name,
    Price,
    Quantity,
    MaxQuantityPerCommand,
    Disabled,
    CreationTime,
}
