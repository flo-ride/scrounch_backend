use crate::{
    mutation::Mutation,
    r#macro::{cache_mdel, cache_set},
    Connection,
};
use ::entity::{location, location::Entity as Location};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_location(
        conn: &Connection,
        form_data: location::Model,
    ) -> Result<location::Model, DbErr> {
        let result = location::ActiveModel {
            id: Set(form_data.id),
            name: Set(form_data.name),
            category: Set(form_data.category),
            creation_time: Set(chrono::offset::Local::now().into()),
            disabled: Set(false),
        }
        .insert(&conn.db_connection)
        .await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            let id = form_data.id.to_string();
            cache_set!(conn, format!("location:{id}"), model, 60 * 15);
            cache_mdel!(conn, "locations");
        }

        result
    }

    pub async fn update_location(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: location::Model,
    ) -> Result<location::Model, DbErr> {
        let result = location::ActiveModel {
            id: Set(id),
            name: Set(form_data.name),
            category: Set(form_data.category),
            creation_time: NotSet,
            disabled: Set(form_data.disabled),
        }
        .update(&conn.db_connection)
        .await;

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
            use fred::{bytes::Bytes, interfaces::KeysInterface};
            if let Some(cache) = &conn.cache_connection {
                let _ = cache.del::<Bytes, _>(format!("location:{id}")).await;
            }
            cache_mdel!(conn, "locations");
        }

        result
    }

    pub async fn delete_all_locations(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Location::delete_many().exec(db).await
    }
}
