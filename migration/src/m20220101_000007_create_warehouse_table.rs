use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(string_null(Warehouse::Name))
                    .col(uuid_null(Warehouse::Parent))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Warehouse::Table, Warehouse::Parent)
                            .to(Warehouse::Table, Warehouse::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
    Parent,
}
