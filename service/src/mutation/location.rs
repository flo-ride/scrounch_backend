#[cfg(feature = "cache")]
use crate::r#macro::{cache_del, cache_mdel, cache_set};
use crate::{Connection, mutation::Mutation};
use ::entity::models::{location, location::Entity as Location};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_location<M: IntoActiveModel<location::ActiveModel>>(
        conn: &Connection,
        form_data: M,
    ) -> Result<location::Model, DbErr> {
        let form_data = form_data.into_active_model();

        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            let id = model.id.to_string();
            cache_set!(conn, format!("location:{id}"), model, 60 * 15);
            cache_mdel!(conn, "locations");
        }

        result
    }

    pub async fn update_location<M: IntoActiveModel<location::ActiveModel>>(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: M,
    ) -> Result<location::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.id = Set(id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            cache_set!(conn, format!("location:{id}"), model, 60 * 15);
            cache_mdel!(conn, "locations");
        }

        result
    }

    pub async fn delete_location<S: Into<String>>(
        conn: &Connection,
        id: S,
    ) -> Result<DeleteResult, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        let location: location::ActiveModel = Location::find_by_id(uuid)
            .one(&conn.db_connection)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find location: \"{id}\"")))
            .map(Into::into)?;

        let result = location.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(conn, format!("location:{id}"));
            cache_mdel!(conn, "locations");
        }

        result
    }

    pub async fn delete_all_locations(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Location::delete_many().exec(db).await
    }
}
