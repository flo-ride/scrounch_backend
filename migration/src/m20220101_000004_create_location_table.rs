use sea_orm::{DbBackend, EnumIter, Iterable};
use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
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
                            .as_enum(LocationCategory)
                            .values(LocationCategoryVariant::iter())
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(Location::Table)
                    .if_not_exists()
                    .col(uuid(Location::Id).primary_key())
                    .col(string_uniq(Location::Name))
                    .col(
                        timestamp_with_time_zone(Location::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(enumeration_null(
                        Location::Category,
                        LocationCategory,
                        LocationCategoryVariant::iter(),
                    ))
                    .col(boolean(Location::Disabled).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Location::Table).to_owned())
            .await?;

        let db = manager.get_connection();
        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .drop_type(Type::drop().name(LocationCategory).to_owned())
                    .await?;
            }
        }
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Location {
    Table,
    Id,
    Name,
    CreatedAt,
    Category,
    Disabled,
}

#[derive(DeriveIden)]
pub struct LocationCategory;

#[derive(DeriveIden, EnumIter)]
pub enum LocationCategoryVariant {
    Dispenser,
    Room,
}
