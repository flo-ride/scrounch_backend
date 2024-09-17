use crate::mutation::Mutation;
use ::entity::{user, user::Entity as User};
use sea_orm::*;
use sqlx::types::Uuid;

impl Mutation {
    pub async fn create_user(db: &DbConn, form_data: user::Model) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            id: Set(form_data.id),
            email: Set(form_data.email),
            name: Set(form_data.name),
            username: Set(form_data.username),
        }
        .insert(db)
        .await
    }

    pub async fn update_user<S: Into<String>>(
        db: &DbConn,
        id: S,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        let user: user::ActiveModel = User::find_by_id(uuid)
            .one(db)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find user: \"{id}\"")))
            .map(Into::into)?;

        user::ActiveModel {
            id: user.id,
            email: Set(form_data.email),
            username: Set(form_data.username),
            name: Set(form_data.name),
        }
        .update(db)
        .await
    }

    pub async fn delete_user<S: Into<String>>(db: &DbConn, id: S) -> Result<DeleteResult, DbErr> {
        let id = id.into();
        let uuid = Uuid::try_parse(&id)
            .map_err(|e| DbErr::Custom(format!("Could not Serialise given id: \"{id}\" - {e}")))?;

        let user: user::ActiveModel = User::find_by_id(uuid)
            .one(db)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find user: \"{id}\"")))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }
}
