//! Defines the `User` entity, representing a user in the database schema.
//!
//! This entity is managed by SeaORM and includes fields that map to columns
//! in the `user` table. It also defines behavior for the `User` entity,
//! such as pre-save checks for banning status.
//!
//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};

/// Represents a user record in the database.
///
/// The `User` entity includes fields for basic user information, admin status,
/// timestamps for creation and last access, as well as an indicator for whether
/// the user is banned.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    /// Unique identifier for the user.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The email address associated with the user.
    pub email: String,

    /// The full name of the user.
    pub name: String,

    /// The username chosen by the user.
    pub username: String,

    /// Indicates if the user has administrative privileges.
    pub is_admin: bool,

    /// Timestamp of when the user was created in the database.
    pub creation_time: DateTimeWithTimeZone,

    /// Timestamp of the user's last access.
    pub last_access_time: DateTimeWithTimeZone,

    /// Indicates if the user is currently banned.
    pub is_banned: bool,
}

/// Enum defining the relationships associated with the `User` entity.
///
/// This enum is currently empty but can be extended to define relationships with
/// other entities, such as orders or profiles.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// Custom behavior for the `User` ActiveModel.
///
/// This implementation includes a `before_save` method to automatically
/// remove administrative privileges if a user is marked as banned.
impl ActiveModelBehavior for ActiveModel {
    /// Executes actions before saving the user entity.
    ///
    /// If a user is marked as banned (`is_banned` is `true`), their admin privileges
    /// (`is_admin`) are automatically revoked.
    fn before_save<'life0, 'async_trait, C>(
        mut self,
        _db: &'life0 C,
        _insert: bool,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, DbErr>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        C: ConnectionTrait,
        C: 'async_trait,
        'life0: 'async_trait,
        Self: core::marker::Send + 'async_trait,
    {
        Box::pin(async move {
            if let Set(true) = self.is_banned {
                self.is_admin = Set(false)
            }
            Ok(self)
        })
    }
}
