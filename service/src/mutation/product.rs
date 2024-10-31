use crate::{
    mutation::Mutation,
    r#macro::{cache_del, cache_mdel, cache_set},
    Connection,
};
use ::entity::models::product::{self, ActiveModel, Entity as Product};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_product<M: IntoActiveModel<ActiveModel>>(
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

    pub async fn update_product(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: product::Model,
    ) -> Result<product::Model, DbErr> {
        let result = product::ActiveModel {
            id: Set(id),
            image: Set(form_data.image),
            name: Set(form_data.name),
            price: Set(form_data.price),
            max_quantity_per_command: Set(form_data.max_quantity_per_command),
            sma_code: Set(form_data.sma_code),
            creation_time: NotSet,
            disabled: Set(form_data.disabled),
        }
        .update(&conn.db_connection)
        .await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            cache_set!(conn, format!("product:{id}"), model, 60 * 15);
            cache_mdel!(conn, "products");
        }

        result
    }

    pub async fn delete_product<S: Into<String>>(
        conn: &Connection,
        id: S,
    ) -> Result<DeleteResult, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        let product: product::ActiveModel = Product::find_by_id(uuid)
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
