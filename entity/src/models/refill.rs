//! `SeaORM` entity representing the `refill` table. This entity defines the
//! structure of refills, including unique identifiers, timestamps, amounts
//! in euros and "epicoin", and an active state marker.

use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};

use super::sea_orm_active_enums::Currency;

/// Represents the `refill` entity in the database, detailing refill transactions
/// including pricing, credit amount, and currency information.
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    proc::DeriveToFilterQuery,
    proc::DeriveToSortQuery,
)]
#[sea_orm(table_name = "refill")]
pub struct Model {
    /// Unique identifier for the refill transaction. Primary key, non-auto-incrementing.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// Optional name or label for the refill transaction.
    pub name: Option<String>,

    /// Timestamp indicating when the refill transaction was created.
    #[sea_orm(filter_override = "chrono::DateTime<chrono::Utc>", filter_plus_order)]
    pub created_at: DateTimeWithTimeZone,

    /// Price of the refill transaction, stored as a decimal with up to 10 digits
    /// and 2 decimal places.
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub price: Decimal,

    /// Currency type for the refill transaction price.
    #[sea_orm(filter_override = "crate::request::r#enum::CurrencyRequest")]
    pub price_currency: Currency,

    /// Credit amount awarded in this refill transaction, stored as a decimal
    /// with up to 10 digits and 2 decimal places.
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub credit: Decimal,

    /// Currency type for the refill transaction credit.
    #[sea_orm(filter_override = "crate::request::r#enum::CurrencyRequest")]
    pub credit_currency: Currency,

    /// Indicates if the refill transaction is hidden.
    #[sea_orm(filter_single)]
    pub hidden: bool,

    /// Indicates if the refill transaction is disabled.
    #[sea_orm(filter_single)]
    pub disabled: bool,
}

/// Enum representing relationships for the `refill` entity.
/// Currently, there are no relationships defined for this entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// Custom behavior for the `ActiveModel` of the `refill` entity.
/// By default, SeaORM provides basic behavior, so no custom behavior is defined here.
impl ActiveModelBehavior for ActiveModel {
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
            if let Set(price) = self.price {
                if price.is_zero() || price.is_sign_negative() {
                    return Err(DbErr::Custom(
                        "Price cannot be null or negative".to_string(),
                    ));
                }
            }

            if let Set(price) = self.credit {
                if price.is_zero() || price.is_sign_negative() {
                    return Err(DbErr::Custom(
                        "Credit cannot be null or negative".to_string(),
                    ));
                }
            }

            // An hidden location MUST be disabled
            if let Set(true) = self.hidden {
                self.disabled = Set(true);
            }

            // An non disabled location CANNOT be hidden
            if let Set(false) = self.disabled {
                self.hidden = Set(false);
            }

            Ok(self)
        })
    }
}
