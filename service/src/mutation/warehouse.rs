#[cfg(feature = "cache")]
use crate::r#macro::{cache_del, cache_mdel, cache_set};
use crate::{mutation::Mutation, Connection};
use ::entity::models::{
    prelude::{Warehouse, WarehouseProducts, WarehouseRecipes},
    warehouse, warehouse_products, warehouse_recipes,
};
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

    pub async fn create_warehouse_products<M: IntoActiveModel<warehouse_products::ActiveModel>>(
        conn: &Connection,
        warehouse_id: uuid::Uuid,
        form_data: M,
    ) -> Result<warehouse_products::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.warehouse_id = Set(warehouse_id);
        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(_model) = &result {
            cache_mdel!(conn, "warehouse_products:{warehouse_id}");
        }

        result
    }

    pub async fn update_warehouse_product<M: IntoActiveModel<warehouse_products::ActiveModel>>(
        conn: &Connection,
        warehouse_id: uuid::Uuid,
        product_id: uuid::Uuid,
        form_data: M,
    ) -> Result<warehouse_products::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.warehouse_id = Set(warehouse_id);
        form_data.product_id = Set(product_id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(_model) = &result {
            cache_del!(
                conn,
                format!("warehouse_product:{warehouse_id}/{product_id}")
            );
            cache_mdel!(conn, format!("warehouse_products:{warehouse_id}"));
        }

        result
    }

    pub async fn delete_warehouse_product(
        conn: &Connection,
        warehouse_id: uuid::Uuid,
        product_id: uuid::Uuid,
    ) -> Result<DeleteResult, DbErr> {
        let warehouse: warehouse_products::ActiveModel =
            WarehouseProducts::find_by_id((warehouse_id, product_id))
                .one(&conn.db_connection)
                .await?
                .ok_or(DbErr::Custom(format!(
                    "Cannot find Warehouse/Products: \"{warehouse_id}\"/\"{product_id}\""
                )))
                .map(Into::into)?;

        let result = warehouse.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(
                conn,
                format!("warehouse_product:{warehouse_id}/{product_id}")
            );
            cache_mdel!(conn, format!("warehouse_products:{warehouse_id}"));
        }

        result
    }

    pub async fn create_warehouse_recipes<M: IntoActiveModel<warehouse_recipes::ActiveModel>>(
        conn: &Connection,
        warehouse_id: uuid::Uuid,
        form_data: M,
    ) -> Result<warehouse_recipes::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.warehouse_id = Set(warehouse_id);
        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(_model) = &result {
            cache_mdel!(conn, "warehouse_recipes:{warehouse_id}");
        }

        result
    }

    pub async fn update_warehouse_recipe<M: IntoActiveModel<warehouse_recipes::ActiveModel>>(
        conn: &Connection,
        warehouse_id: uuid::Uuid,
        recipe_id: uuid::Uuid,
        form_data: M,
    ) -> Result<warehouse_recipes::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.warehouse_id = Set(warehouse_id);
        form_data.recipe_id = Set(recipe_id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(_model) = &result {
            cache_del!(conn, format!("warehouse_recipe:{warehouse_id}/{recipe_id}"));
            cache_mdel!(conn, format!("warehouse_recipes:{warehouse_id}"));
        }

        result
    }

    pub async fn delete_warehouse_recipe(
        conn: &Connection,
        warehouse_id: uuid::Uuid,
        recipe_id: uuid::Uuid,
    ) -> Result<DeleteResult, DbErr> {
        let warehouse: warehouse_recipes::ActiveModel =
            WarehouseRecipes::find_by_id((warehouse_id, recipe_id))
                .one(&conn.db_connection)
                .await?
                .ok_or(DbErr::Custom(format!(
                    "Cannot find Warehouse/Recipes: \"{warehouse_id}\"/\"{recipe_id}\""
                )))
                .map(Into::into)?;

        let result = warehouse.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(conn, format!("warehouse_recipe:{warehouse_id}/{recipe_id}"));
            cache_mdel!(conn, format!("warehouse_recipes:{warehouse_id}"));
        }

        result
    }
}
