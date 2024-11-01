pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20220101_000002_create_product_table;
mod m20220101_000003_add_is_banned;
mod m20220101_000004_remove_quantity_from_product;
mod m20220101_000005_create_location_table;
mod m20220101_000006_create_refill_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20220101_000002_create_product_table::Migration),
            Box::new(m20220101_000003_add_is_banned::Migration),
            Box::new(m20220101_000004_remove_quantity_from_product::Migration),
            Box::new(m20220101_000005_create_location_table::Migration),
            Box::new(m20220101_000006_create_refill_table::Migration),
        ]
    }
}
