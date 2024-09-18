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
        db: &Connection,
        id: S,
    ) -> Result<Option<user::Model>, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        User::find_by_id(uuid).one(&db.db_connection).await
    }
}
