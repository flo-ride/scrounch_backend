use crate::{mutation::Mutation, Connection};
use ::entity::{product, product::Entity as Product};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_product(
        conn: &Connection,
        form_data: product::Model,
    ) -> Result<product::Model, DbErr> {
        let result = product::ActiveModel {
            id: Set(form_data.id),
            image: Set(form_data.image),
            name: Set(form_data.name),
            price: Set(form_data.price),
            quantity: Set(form_data.quantity),
            max_quantity_per_command: Set(form_data.max_quantity_per_command),
            creation_time: Set(chrono::offset::Local::now().into()),
            disabled: Set(false),
        }
        .insert(&conn.db_connection)
        .await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            let id = form_data.id.to_string();
            crate::cache_set!(conn, format!("product:{id}"), model, 60 * 15);
        }

        result
    }

    pub async fn update_product(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: product::Model,
    ) -> Result<product::Model, DbErr> {
        let product: product::ActiveModel = Product::find_by_id(id)
            .one(&conn.db_connection)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find product: \"{id}\"")))
            .map(Into::into)?;

        let result = product::ActiveModel {
            id: product.id,
            image: Set(form_data.image),
            name: Set(form_data.name),
            price: Set(form_data.price),
            quantity: Set(form_data.quantity),
            max_quantity_per_command: Set(form_data.max_quantity_per_command),
            creation_time: NotSet,
            disabled: Set(false),
        }
        .update(&conn.db_connection)
        .await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            crate::cache_set!(conn, format!("product:{id}"), model, 60 * 15);
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
            use fred::{bytes::Bytes, interfaces::KeysInterface};
            if let Some(cache) = &conn.cache_connection {
                let _ = cache.del::<Bytes, _>(format!("product:{id}")).await;
            }
        }

        result
    }

    pub async fn delete_all_products(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Product::delete_many().exec(db).await
    }
}
