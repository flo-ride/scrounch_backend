//! User mutation services for the `scrounch_backend` application.
//!
//! This module defines services and functions related to modifying user data in the application.
//! It encapsulates the logic for creating, updating, and deleting user records in the database.
//! These services provide a layer of abstraction over database mutations, ensuring that changes
//! to user data are handled efficiently and consistently.

use crate::{mutation::Mutation, Connection};
use ::entity::{user, user::Entity as User};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_user(
        conn: &Connection,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            id: Set(form_data.id),
            email: Set(form_data.email),
            name: Set(form_data.name),
            username: Set(form_data.username),
        }
        .insert(&conn.db_connection)
        .await
    }

    pub async fn update_user<S: Into<String>>(
        conn: &Connection,
        id: S,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        let user: user::ActiveModel = User::find_by_id(uuid)
            .one(&conn.db_connection)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find user: \"{id}\"")))
            .map(Into::into)?;

        user::ActiveModel {
            id: user.id,
            email: Set(form_data.email),
            username: Set(form_data.username),
            name: Set(form_data.name),
        }
        .update(&conn.db_connection)
        .await
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

        user.delete(&conn.db_connection).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }
}
