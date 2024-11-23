use extension::postgres::Type;
use sea_orm::{DbBackend, EnumIter, Iterable};
use sea_orm_migration::prelude::*;

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
                        extension::postgres::Type::create()
                            .as_enum(Currency)
                            .values(CurrencyVariant::iter())
                            .to_owned(),
                    )
                    .await?;

                manager
                    .create_type(
                        extension::postgres::Type::create()
                            .as_enum(Unit)
                            .values(UnitVariant::iter())
                            .to_owned(),
                    )
                    .await?;
            }
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .drop_type(Type::drop().name(Currency).to_owned())
                    .await?;

                manager
                    .drop_type(Type::drop().name(Unit).to_owned())
                    .await?;
            }
        }
        Ok(())
    }
}

#[derive(DeriveIden)]
pub struct Currency;

#[derive(DeriveIden, EnumIter)]
pub enum CurrencyVariant {
    Euro,
    Epicoin,
}

#[derive(DeriveIden)]
pub struct Unit;

#[derive(DeriveIden, EnumIter)]
pub enum UnitVariant {
    /// Represents a single unit or piece (e.g., an item).
    Unit,
    /// Represents weight in grams (as the base unit for mass).
    Gram,
    /// Represents volume in liters (as the base unit for volume).
    Liter,
    /// Represents length in meters (as the base unit for distance).
    Meter,
}
