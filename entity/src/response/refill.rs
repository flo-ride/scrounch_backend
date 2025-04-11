//! This module contains response structures for refill-related API responses.
//! It defines the format of data returned to clients regarding refills.

use rust_decimal::{Decimal, Error as DecimalError};
use serde_with::skip_serializing_none;

use crate::error::impl_from_error_to_string;

use super::r#enum::CurrencyResponse;

/// Enum representing errors that can occur during refill response construction.
#[derive(Debug, PartialEq, Clone)]
pub enum RefillResponseError {
    /// Error when the amount in euros could not be converted to a floating-point representation.
    AmountInEuroCannotBeConverted(Decimal, DecimalError),

    /// Error when the amount in epicoin could not be converted to a floating-point representation.
    AmountInEpicoinCannotBeConverted(Decimal, DecimalError),
}

impl std::error::Error for RefillResponseError {}

/// Formats error messages for the `RefillRequestError` enum.
impl std::fmt::Display for RefillResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AmountInEuroCannotBeConverted(price, err) => {
                write!(
                    f,
                    "Amount in Euro \"{price}\" cannot be converted in: {err}"
                )
            }
            Self::AmountInEpicoinCannotBeConverted(price, err) => {
                write!(
                    f,
                    "Amount in Epicoin \"{price}\" cannot be converted in: {err}"
                )
            }
        }
    }
}

impl_from_error_to_string!(RefillResponseError, InternalError);

/// Represents a response containing refill information returned by the API.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct RefillResponse {
    /// Unique identifier for the refill.
    pub id: uuid::Uuid,

    /// The full name of the refill.
    pub name: Option<String>,

    /// Price of the refill.
    pub price: f64,

    /// Currency type for the refill price.
    pub price_currency: CurrencyResponse,

    /// Credit of the refill.
    pub credit: f64,

    /// Currency type for the refill credit.
    pub credit_currency: CurrencyResponse,

    /// The timestamp indicating when the refill was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Indicates whether the refill is currently hidden.
    pub hidden: Option<bool>,

    /// Indicates whether the refill is currently disabled.
    pub disabled: bool,
}

/// Converts a `Model` from the refill module to a `RefillResponse`.
impl TryFrom<crate::models::refill::Model> for RefillResponse {
    type Error = RefillResponseError;

    fn try_from(value: crate::models::refill::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name,
            price: match value.price.try_into() {
                Ok(price) => price,
                Err(err) => {
                    return Err(Self::Error::AmountInEuroCannotBeConverted(value.price, err));
                }
            },
            price_currency: value.price_currency.into(),
            credit: match value.credit.try_into() {
                Ok(price) => price,
                Err(err) => {
                    return Err(Self::Error::AmountInEpicoinCannotBeConverted(
                        value.credit,
                        err,
                    ));
                }
            },
            credit_currency: value.credit_currency.into(),
            hidden: match value.hidden {
                true => Some(true),
                false => None,
            },
            disabled: value.disabled,
            created_at: value.created_at.into(),
        })
    }
}

/// Represents a response containing a list of refills returned by the API.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct RefillListResponse {
    /// The total number of pages available for refill results.
    pub total_page: u64,

    /// The current page number being viewed.
    pub current_page: u64,

    /// A list of refill responses containing refill details.
    pub refills: Vec<RefillResponse>,
}
