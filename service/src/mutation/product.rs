#[cfg(feature = "cache")]
use crate::r#macro::{cache_del, cache_mdel, cache_set};
use crate::{Connection, mutation::Mutation};
use ::entity::models::product::{self, Entity as Product};
use sea_orm::*;

impl Mutation {
    pub async fn create_product<M: IntoActiveModel<product::ActiveModel>>(
        conn: &Connection,
        form_data: M,
    ) -> Result<product::Model, DbErr> {
        let form_data = form_data.into_active_model();
        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            let id = model.id;
            cache_set!(conn, format!("product:{id}"), model, 60 * 15);
            cache_mdel!(conn, "products");
        }

        result
    }

    pub async fn update_product<M: IntoActiveModel<product::ActiveModel>>(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: M,
    ) -> Result<product::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.id = Set(id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            cache_set!(conn, format!("product:{id}"), model, 60 * 15);
            cache_mdel!(conn, "products");
        }

        result
    }

    pub async fn delete_product(conn: &Connection, id: uuid::Uuid) -> Result<DeleteResult, DbErr> {
        let product: product::ActiveModel = Product::find_by_id(id)
            .one(&conn.db_connection)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find product: \"{id}\"")))
            .map(Into::into)?;

        let result = product.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(conn, format!("product:{id}"));
            cache_mdel!(conn, "products");
        }

        result
    }

    pub async fn delete_all_products(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Product::delete_many().exec(db).await
    }
}
