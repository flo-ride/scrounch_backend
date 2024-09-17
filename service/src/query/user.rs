use crate::query::Query;
use ::entity::{user, user::Entity as User};
use sea_orm::*;
use sqlx::types::Uuid;

impl Query {
    pub async fn find_user_by_id<S: Into<String>>(
        db: &DbConn,
        id: S,
    ) -> Result<Option<user::Model>, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        User::find_by_id(uuid).one(db).await
    }
}
