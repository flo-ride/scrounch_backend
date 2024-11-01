//! # Refill Request Models and Error Handling
//! This module defines the structures and associated logic for handling refill requests in the system,
//! including both the creation and editing of refills.

use crate::models::refill::ActiveModel;
use rust_decimal::{Decimal, Error as DecimalError};
use sea_orm::ActiveValue::{NotSet, Set};

/// The maximum allowed length for a refill name.
/// This constraint ensures that names remain concise and standardized in the database.
pub const REFILL_NAME_MAX_LENGTH: usize = 32;

/// Enum representing potential errors in the refill request validation process.
#[derive(Debug, PartialEq, Clone)]
pub enum RefillRequestError {
    /// Error when the refill name is empty.
    NameCannotBeEmpty,
    /// Error when the refill name exceeds the allowed maximum length.
    NameCannotBeLongerThan(String, usize),
    /// Error when the amount in euros is zero or negative.
    AmountInEuroCannotBeNullOrNegative(f64),
    /// Error when the amount in euros cannot be converted to a `Decimal` type.
    AmountInEuroCannotBeConvertedInDecimal(String, DecimalError),
    /// Error when the amount in epicoin is zero.
    AmountInEpicoinCannotBeNull,
}

impl std::error::Error for RefillRequestError {}

/// Formats error messages for the `RefillRequestError` enum.
impl std::fmt::Display for RefillRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameCannotBeEmpty => write!(f, "Name cannot be empty"),
            Self::NameCannotBeLongerThan(name, max) => {
                write!(f, "Name \"{name}\" is longer than {max} characters")
            }
            Self::AmountInEuroCannotBeNullOrNegative(price) => {
                write!(f, "Amount in Euro \"{price}\" cannot be null or negative")
            }
            Self::AmountInEuroCannotBeConvertedInDecimal(price, err) => {
                write!(
                    f,
                    "Amount in Euro \"{price}\" cannot be converted in Decimal: {err}"
                )
            }
            Self::AmountInEpicoinCannotBeNull => {
                write!(f, "Amount in Epicoin cannot be equal to 0")
            }
        }
    }
}

/// Request structure for creating a new refill, including validation rules.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewRefillRequest {
    /// Name of the refill, required and validated for length.
    pub name: Option<String>,
    /// Amount in euros, required and validated for non-negativity.
    pub amount_in_euro: f64,
    /// Amount in epicoin, required and validated for non-zero value.
    pub amount_in_epicoin: u64,
}

/// Converts `NewRefillRequest` into `ActiveModel` with validation.
/// Errors are returned if validation fails.
impl TryFrom<NewRefillRequest> for ActiveModel {
    type Error = RefillRequestError;

    fn try_from(value: NewRefillRequest) -> Result<Self, Self::Error> {
        Ok(ActiveModel {
            id: Set(uuid::Uuid::new_v4()), // Automatically generate a new UUID for the ID.
            name: match value.name {
                Some(name) => {
                    if name.is_empty() {
                        return Err(Self::Error::NameCannotBeEmpty);
                    }
                    if name.len() > REFILL_NAME_MAX_LENGTH {
                        return Err(Self::Error::NameCannotBeLongerThan(
                            name,
                            REFILL_NAME_MAX_LENGTH,
                        ));
                    }
                    Set(Some(name))
                }
                None => NotSet,
            },
            amount_in_euro: {
                let price = value.amount_in_euro;
                if price <= 0.0 {
                    return Err(Self::Error::AmountInEuroCannotBeNullOrNegative(price));
                }
                let price = price.to_string();
                match Decimal::from_str_exact(&price) {
                    Ok(price) => Set(price),
                    Err(err) => {
                        return Err(Self::Error::AmountInEuroCannotBeConvertedInDecimal(
                            price, err,
                        ))
                    }
                }
            },
            amount_in_epicoin: {
                let price = value.amount_in_epicoin;
                if price == 0 {
                    return Err(Self::Error::AmountInEpicoinCannotBeNull);
                }
                Set(Decimal::from(price))
            },
            disabled: Set(false), // Default value for disabled is set to false.
            creation_time: Set(chrono::offset::Local::now().into()), // Capture current time as creation timestamp.
        })
    }
}

/// Structure representing a request to edit an existing refill entry.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditRefillRequest {
    /// Optional new name for the refill.
    pub name: Option<Option<String>>,
    /// Optional new amount in euros for the refill.
    pub amount_in_euro: Option<f64>,
    /// Optional new amount in epicoin for the refill.
    pub amount_in_epicoin: Option<u64>,
    /// Optional new disabled status for the refill.
    pub disabled: Option<bool>,
}

/// Converts `EditRefillRequest` into `ActiveModel` with validation.
/// Only fields present in the request are updated.
impl TryFrom<EditRefillRequest> for ActiveModel {
    type Error = RefillRequestError;

    fn try_from(value: EditRefillRequest) -> Result<Self, Self::Error> {
        Ok(ActiveModel {
            id: NotSet, // ID is not updated during edits.
            name: match value.name {
                Some(name_opt) => match name_opt {
                    Some(name) => {
                        if name.is_empty() {
                            return Err(Self::Error::NameCannotBeEmpty);
                        }
                        if name.len() > REFILL_NAME_MAX_LENGTH {
                            return Err(Self::Error::NameCannotBeLongerThan(
                                name,
                                REFILL_NAME_MAX_LENGTH,
                            ));
                        }
                        Set(Some(name))
                    }
                    None => Set(None),
                },
                None => NotSet,
            },
            amount_in_euro: match value.amount_in_euro {
                Some(price) => {
                    if price <= 0.0 {
                        return Err(Self::Error::AmountInEuroCannotBeNullOrNegative(price));
                    }
                    let price = price.to_string();
                    match Decimal::from_str_exact(&price) {
                        Ok(price) => Set(price),
                        Err(err) => {
                            return Err(Self::Error::AmountInEuroCannotBeConvertedInDecimal(
                                price, err,
                            ))
                        }
                    }
                }
                None => NotSet,
            },
            amount_in_epicoin: match value.amount_in_epicoin {
                Some(price) => {
                    if price == 0 {
                        return Err(Self::Error::AmountInEpicoinCannotBeNull);
                    }
                    Set(Decimal::from(price))
                }
                None => NotSet,
            },
            disabled: match value.disabled {
                Some(disabled) => Set(disabled),
                None => NotSet,
            },
            creation_time: NotSet, // Creation time is not updated during edits.
        })
    }
}
