//! User query services for the `scrounch_backend` application.
//!
//! This module defines services and functions related to querying user data in the application.
//! It encapsulates the logic for retrieving user information from the database based on various
//! criteria such as user ID, username, or email. These services provide a layer of abstraction
//! over database interactions, allowing for efficient and consistent data retrieval related to users.

use crate::{
    query::Query,
    r#macro::{cache_get, cache_mget, cache_mset, cache_set},
    Connection,
};
use ::entity::{user, user::Entity as User};
use sea_orm::*;

impl Query {
    pub async fn find_user_by_id(
        conn: &Connection,
        id: uuid::Uuid,
    ) -> Result<Option<user::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_get!(conn, format!("user:{id}"), user::Model);

        let result = User::find_by_id(id).one(&conn.db_connection).await?;

        #[cfg(feature = "cache")]
        if let Some(model) = &result {
            cache_set!(conn, format!("user:{id}"), model, 60 * 15);
        }

        Ok(result)
    }

    pub async fn list_users_with_condition<
        F: sea_query::IntoCondition + std::fmt::Debug + Clone,
        A: Into<u64> + Copy,
        P: Into<u64> + Copy,
    >(
        conn: &Connection,
        filter: F,
        page: A,
        per_page: P,
    ) -> Result<Vec<user::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_mget!(
            conn,
            format!("users:{filter:?}-{}/{}", page.into(), per_page.into()),
            user::Model
        );

        let result = User::find()
            .filter(filter.clone())
            .paginate(&conn.db_connection, per_page.into())
            .fetch_page(page.into())
            .await?;

        #[cfg(feature = "cache")]
        cache_mset!(
            conn,
            format!("users:{filter:?}-{}/{}", page.into(), per_page.into()),
            result,
            60 * 15,
            "user:"
        );

        Ok(result)
    }

    pub async fn count_users_with_condition<F: sea_query::IntoCondition>(
        conn: &Connection,
        filter: F,
    ) -> Result<u64, DbErr> {
        User::find().filter(filter).count(&conn.db_connection).await
    }
}
