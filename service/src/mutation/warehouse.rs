#[cfg(feature = "cache")]
use crate::r#macro::{cache_del, cache_mdel, cache_set};
use crate::{mutation::Mutation, Connection};
use ::entity::models::warehouse::{self, Entity as Warehouse};
use sea_orm::*;

impl Mutation {
    pub async fn create_warehouse<M: IntoActiveModel<warehouse::ActiveModel>>(
        conn: &Connection,
        form_data: M,
    ) -> Result<warehouse::Model, DbErr> {
        let form_data = form_data.into_active_model();
        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            let id = model.id;
            cache_set!(conn, format!("warehouse:{id}"), model, 60 * 15);
            cache_mdel!(conn, "warehouses");
        }

        result
    }

    pub async fn update_warehouse<M: IntoActiveModel<warehouse::ActiveModel>>(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: M,
    ) -> Result<warehouse::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.id = Set(id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            cache_set!(conn, format!("warehouse:{id}"), model, 60 * 15);
            cache_mdel!(conn, "warehouses");
        }

        result
    }

    pub async fn delete_warehouse(
        conn: &Connection,
        id: uuid::Uuid,
    ) -> Result<DeleteResult, DbErr> {
        let warehouse: warehouse::ActiveModel = Warehouse::find_by_id(id)
            .one(&conn.db_connection)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find warehouse: \"{id}\"")))
            .map(Into::into)?;

        let result = warehouse.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(conn, format!("warehouse:{id}"));
            cache_mdel!(conn, "warehouses");
        }

        result
    }

    pub async fn delete_all_warehouses(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Warehouse::delete_many().exec(db).await
    }
}
