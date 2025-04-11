#[cfg(feature = "cache")]
use crate::r#macro::{cache_del, cache_mdel, cache_set};
use crate::{Connection, mutation::Mutation};
use ::entity::models::{refill, refill::Entity as Refill};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_refill<M: IntoActiveModel<refill::ActiveModel>>(
        conn: &Connection,
        form_data: M,
    ) -> Result<refill::Model, DbErr> {
        let form_data = form_data.into_active_model();

        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            let id = model.id.to_string();
            cache_set!(conn, format!("refill:{id}"), model, 60 * 15);
            cache_mdel!(conn, "refills");
        }

        result
    }

    pub async fn update_refill<M: IntoActiveModel<refill::ActiveModel>>(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: M,
    ) -> Result<refill::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.id = ActiveValue::Set(id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            cache_set!(conn, format!("refill:{id}"), model, 60 * 15);
            cache_mdel!(conn, "refills");
        }

        result
    }

    pub async fn delete_refill<S: Into<String>>(
        conn: &Connection,
        id: S,
    ) -> Result<DeleteResult, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        let refill: refill::ActiveModel = Refill::find_by_id(uuid)
            .one(&conn.db_connection)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find refill: \"{id}\"")))
            .map(Into::into)?;

        let result = refill.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(conn, format!("refill:{id}"));
            cache_mdel!(conn, "refills");
        }

        result
    }

    pub async fn delete_all_refills(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Refill::delete_many().exec(db).await
    }
}
