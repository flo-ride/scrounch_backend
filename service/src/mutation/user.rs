//! User mutation services for the `scrounch_backend` application.
//!
//! This module defines services and functions related to modifying user data in the application.
//! It encapsulates the logic for creating, updating, and deleting user records in the database.
//! These services provide a layer of abstraction over database mutations, ensuring that changes
//! to user data are handled efficiently and consistently.

use crate::{
    mutation::Mutation,
    r#macro::{cache_del, cache_mdel, cache_set},
    Connection,
};
use ::entity::models::{user, user::Entity as User};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_user(
        conn: &Connection,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let result = user::ActiveModel {
            id: Set(form_data.id),
            email: Set(form_data.email),
            name: Set(form_data.name),
            username: Set(form_data.username),
            is_banned: Set(form_data.is_banned),
            is_admin: Set(form_data.is_admin),
            creation_time: Set(chrono::offset::Local::now().into()),
            last_access_time: Set(chrono::offset::Local::now().into()),
        }
        .insert(&conn.db_connection)
        .await;

        #[cfg(feature = "cache")]
        if let Ok(model) = &result {
            let id = form_data.id.to_string();
            cache_set!(conn, format!("user:{id}"), model, 60 * 15);
            cache_mdel!(conn, "users");
        }

        result
    }

    pub async fn update_user(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let result = user::ActiveModel {
            id: Set(id),
            email: Set(form_data.email),
            username: Set(form_data.username),
            name: Set(form_data.name),
            is_admin: Set(form_data.is_admin),
            is_banned: Set(form_data.is_banned),
            creation_time: NotSet,
            last_access_time: NotSet,
        }
        .update(&conn.db_connection)
        .await;

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
            email: NotSet,
            username: NotSet,
            name: NotSet,
            is_admin: NotSet,
            creation_time: NotSet,
            is_banned: NotSet,
            last_access_time: Set(chrono::offset::Local::now().into()),
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
