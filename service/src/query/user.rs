//! User query services for the `scrounch_backend` application.
//!
//! This module defines services and functions related to querying user data in the application.
//! It encapsulates the logic for retrieving user information from the database based on various
//! criteria such as user ID, username, or email. These services provide a layer of abstraction
//! over database interactions, allowing for efficient and consistent data retrieval related to users.

use crate::{query::Query, Connection};
use ::entity::{user, user::Entity as User};
use sea_orm::*;
use sqlx::types::Uuid;

impl Query {
    pub async fn find_user_by_id<S: Into<String>>(
        conn: &Connection,
        id: S,
    ) -> Result<Option<user::Model>, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        #[cfg(feature = "cache")]
        crate::cache_get!(conn, format!("user:{id}"), user::Model);

        let result = User::find_by_id(uuid).one(&conn.db_connection).await?;

        #[cfg(feature = "cache")]
        if let Some(model) = &result {
            crate::cache_set!(conn, format!("user:{id}"), model, 60 * 15);
        }

        Ok(result)
    }

    pub async fn list_users_with_condition<F: sea_query::IntoCondition>(
        conn: &Connection,
        filter: F,
    ) -> Result<Vec<user::Model>, DbErr> {
        User::find().filter(filter).all(&conn.db_connection).await
    }

    pub async fn count_users_with_condition<F: sea_query::IntoCondition>(
        conn: &Connection,
        filter: F,
    ) -> Result<u64, DbErr> {
        User::find().filter(filter).count(&conn.db_connection).await
    }
}
