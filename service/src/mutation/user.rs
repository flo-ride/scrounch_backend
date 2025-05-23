//! User mutation services for the `scrounch_backend` application.
//!
//! This module defines services and functions related to modifying user data in the application.
//! It encapsulates the logic for creating, updating, and deleting user records in the database.
//! These services provide a layer of abstraction over database mutations, ensuring that changes
//! to user data are handled efficiently and consistently.

#[cfg(feature = "cache")]
use crate::r#macro::{cache_del, cache_mdel, cache_set};
use crate::{Connection, mutation::Mutation};
use ::entity::models::{user, user::Entity as User};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_user<M: IntoActiveModel<user::ActiveModel>>(
        conn: &Connection,
        form_data: M,
    ) -> Result<user::Model, DbErr> {
        let form_data = form_data.into_active_model();

        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            let id = model.id.to_string();
            cache_set!(conn, format!("user:{id}"), model, 60 * 15);
            cache_mdel!(conn, "users");
        }

        result
    }

    pub async fn update_user<M: IntoActiveModel<user::ActiveModel>>(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: M,
    ) -> Result<user::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.id = Set(id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            cache_set!(conn, format!("user:{id}"), model, 60 * 15);
            cache_mdel!(conn, "users");
        }

        result
    }

    pub async fn update_user_last_access_time(
        conn: &Connection,
        id: uuid::Uuid,
    ) -> Result<user::Model, DbErr> {
        let result = user::ActiveModel {
            id: Set(id),
            last_access_at: Set(chrono::offset::Local::now().into()),
            ..Default::default()
        }
        .update(&conn.db_connection)
        .await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            cache_set!(conn, format!("user:{id}"), model, 60 * 15);
        }

        result
    }

    pub async fn delete_user<S: Into<String>>(
        conn: &Connection,
        id: S,
    ) -> Result<DeleteResult, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        let user: user::ActiveModel = User::find_by_id(uuid)
            .one(&conn.db_connection)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find user: \"{id}\"")))
            .map(Into::into)?;

        let result = user.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(conn, format!("user:{id}"));
            cache_mdel!(conn, "users");
        }

        result
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }
}
