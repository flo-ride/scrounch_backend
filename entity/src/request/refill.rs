//! # Refill Request Models and Error Handling
//! This module defines the structures and associated logic for handling refill requests in the system,
//! including both the creation and editing of refills.

use crate::models::refill::ActiveModel;
use rust_decimal::{Decimal, Error as DecimalError};
use sea_orm::ActiveValue::{NotSet, Set};

use super::r#enum::CurrencyRequest;

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
    /// Error when the price is zero or negative.
    PriceCannotBeNullOrNegative(f64),
    /// Error when the price cannot be converted to a `Decimal` type.
    PriceCannotBeConvertedInDecimal(String, DecimalError),
    /// Error when the credit is zero or negative.
    CreditCannotBeNullOrNegative(f64),
    /// Error when the credit cannot be converted to a `Decimal` type.
    CreditCannotBeConvertedInDecimal(String, DecimalError),
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
            Self::PriceCannotBeNullOrNegative(price) => {
                write!(f, "Price \"{price}\" cannot be null or negative")
            }
            Self::PriceCannotBeConvertedInDecimal(price, err) => {
                write!(f, "Price \"{price}\" cannot be converted in Decimal: {err}")
            }
            Self::CreditCannotBeNullOrNegative(credit) => {
                write!(f, "Credit \"{credit}\" cannot be null or negative")
            }
            Self::CreditCannotBeConvertedInDecimal(credit, err) => {
                write!(
                    f,
                    "Credit \"{credit}\" cannot be converted in Decimal: {err}"
                )
            }
        }
    }
}

/// Request structure for creating a new refill, including validation rules.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewRefillRequest {
    /// Name of the refill, required and validated for length.
    pub name: Option<String>,

    /// Amount for buying refill
    pub price: f64,

    /// Currency type for the refill price.
    pub price_currency: CurrencyRequest,

    /// Amount given with refill
    pub credit: f64,

    /// Currency type for the refill credit.
    pub credit_currency: CurrencyRequest,
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
            price: {
                let price = value.price;
                if price <= 0.0 {
                    return Err(Self::Error::PriceCannotBeNullOrNegative(price));
                }
                let price = price.to_string();
                match Decimal::from_str_exact(&price) {
                    Ok(price) => Set(price),
                    Err(err) => {
                        return Err(Self::Error::PriceCannotBeConvertedInDecimal(price, err))
                    }
                }
            },
            price_currency: Set(value.price_currency.into()),
            credit: {
                let credit = value.credit;
                if credit <= 0.0 {
                    return Err(Self::Error::CreditCannotBeNullOrNegative(credit));
                }
                let credit = credit.to_string();
                match Decimal::from_str_exact(&credit) {
                    Ok(credit) => Set(credit),
                    Err(err) => {
                        return Err(Self::Error::CreditCannotBeConvertedInDecimal(credit, err))
                    }
                }
            },
            credit_currency: Set(value.credit_currency.into()),
            disabled: Set(false), // Default value for disabled is set to false.
            created_at: Set(chrono::offset::Local::now().into()), // Capture current time as creation timestamp.
        })
    }
}

/// Structure representing a request to edit an existing refill entry.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditRefillRequest {
    /// Optional new name for the refill.
    pub name: Option<Option<String>>,

    /// Optional new amount for the refill.
    pub price: Option<f64>,

    /// Optional currency type for the new refill price.
    pub price_currency: Option<CurrencyRequest>,

    /// Optional new amount for the refill.
    pub credit: Option<f64>,

    /// Optional currency type for the new refill credit.
    pub credit_currency: Option<CurrencyRequest>,

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
            price: match value.price {
                Some(price) => {
                    if price <= 0.0 {
                        return Err(Self::Error::PriceCannotBeNullOrNegative(price));
                    }
                    let price = price.to_string();
                    match Decimal::from_str_exact(&price) {
                        Ok(price) => Set(price),
                        Err(err) => {
                            return Err(Self::Error::PriceCannotBeConvertedInDecimal(price, err))
                        }
                    }
                }
                None => NotSet,
            },
            price_currency: match value.price_currency {
                Some(currency) => Set(currency.into()),
                None => NotSet,
            },
            credit: match value.credit {
                Some(credit) => {
                    if credit <= 0.0 {
                        return Err(Self::Error::CreditCannotBeNullOrNegative(credit));
                    }
                    let credit = credit.to_string();
                    match Decimal::from_str_exact(&credit) {
                        Ok(credit) => Set(credit),
                        Err(err) => {
                            return Err(Self::Error::CreditCannotBeConvertedInDecimal(credit, err))
                        }
                    }
                }
                None => NotSet,
            },
            credit_currency: match value.credit_currency {
                Some(currency) => Set(currency.into()),
                None => NotSet,
            },
            disabled: match value.disabled {
                Some(disabled) => Set(disabled),
                None => NotSet,
            },
            ..Default::default()
        })
    }
}
