//! Defines request structures and conversion logic for creating and editing product entities.
//!
//! This module provides request models and error handling for product-related operations,
//! including creating new products and editing existing ones. It validates request data
//! and ensures that only valid information is converted into the `ActiveModel` format,
//! which is then used for database transactions. Additionally, it defines custom error
//! types to handle validation failures in a structured manner.

use crate::{error::impl_bad_request_app_error, models::product::ActiveModel};
use rust_decimal::{Decimal, Error as DecimalError};
use sea_orm::ActiveValue::{NotSet, Set};
use std::num::TryFromIntError;

use super::r#enum::{CurrencyRequest, UnitRequest};

/// The maximum allowed length for a product name.
/// This constraint ensures that names remain concise and standardized in the database.
pub const PRODUCT_NAME_MAX_LENGTH: usize = 32;

/// The maximum quantity of a product allowed per command.
/// This limit helps control inventory and prevent excessive ordering in a single transaction.
pub const PRODUCT_MAX_QUANTITY_PER_COMMAND: u64 = 10;

/// Errors specific to product requests, including validation and conversion errors.
#[derive(Debug, PartialEq, Clone, strum_macros::IntoStaticStr)]
pub enum ProductRequestError {
    /// Error when the product name is empty.
    NameCannotBeEmpty,
    /// Error when the product name exceeds the allowed maximum length.
    NameCannotBeLongerThan(String, usize),
    /// Error when the price is negative or zero.
    PriceCannotBeNegativeOrNull(f64),
    /// Error when the price cannot be converted into a `Decimal`.
    PriceCannotBeConvertedInDecimal(String, DecimalError),
    /// Error when the maximum quantity per command exceeds the limit.
    MaxQuantityPerCommandCannotBeBiggerThan(u64, u64),
    /// Error when the maximum quantity per command cannot be converted to an `i16`.
    MaxQuantityPerCommandCannotBeConvertedToI16(u64, TryFromIntError),
    /// Error when the specified image does not exist in S3.
    ImageDoesNotExist(String),
    /// Display order cannot be converted to i32
    DisplayOrderCannotBeConvertedToI32(u64, TryFromIntError),
}
impl std::error::Error for ProductRequestError {}

impl std::fmt::Display for ProductRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductRequestError::NameCannotBeEmpty => write!(f, "Name Cannot be Empty"),
            ProductRequestError::NameCannotBeLongerThan(name, max) => {
                write!(f, "Name \"{name}\" is longer than {max} characters")
            }
            ProductRequestError::PriceCannotBeNegativeOrNull(price) => {
                write!(f, "Price \"{price}\" cannot be null or negative")
            }
            ProductRequestError::PriceCannotBeConvertedInDecimal(price, err) => {
                write!(f, "Price \"{price}\" cannot be converted in Decimal: {err}")
            }
            ProductRequestError::MaxQuantityPerCommandCannotBeBiggerThan(
                max_quantity_per_command,
                max,
            ) => {
                write!(f, "Max Quantity Per Command \"{max_quantity_per_command}\" cannot be bigger than {max}")
            }
            ProductRequestError::MaxQuantityPerCommandCannotBeConvertedToI16(
                max_quantity_per_command,
                err,
            ) => {
                write!(f, "Max Quantity Per Command \"{max_quantity_per_command}\" cannot be converted to i16 {err}")
            }
            ProductRequestError::ImageDoesNotExist(image) => {
                write!(f, "Image \"{image}\" does't exist in S3")
            }
            ProductRequestError::DisplayOrderCannotBeConvertedToI32(display_order, err) => {
                write!(
                    f,
                    "Display Order \"{display_order}\" cannot be converted to i32 {err}"
                )
            }
        }
    }
}

impl_bad_request_app_error!(ProductRequestError);

impl From<ProductRequestError> for crate::error::ErrorResponse {
    fn from(value: ProductRequestError) -> Self {
        let kind: &'static str = value.clone().into();
        Self {
            status: 400,
            error: "Bad Request".to_string(),
            kind: kind.to_string(),
            message: value.to_string(),
        }
    }
}

/// Request structure for creating a new product, including validation rules.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewProductRequest {
    /// Optional image URL or path.
    pub image: Option<String>,
    /// Name of the product, required and validated for length.
    pub name: String,
    /// Price of the product, required and must be positive.
    pub sell_price: f64,
    /// Currency of the product.
    pub sell_price_currency: CurrencyRequest,
    /// If the product is purchasable or if it's just an ingredients, if it's not it's
    /// automatically hidden
    pub purchasable: Option<bool>,
    /// If the product is hidden from user, if true, it's automatically disable
    pub hidden: Option<bool>,
    /// If the product is disabled from user (can't create new order with it)
    pub disabled: Option<bool>,
    /// Represent the unit type of Product, if it's a liquid -> Liter, etc..., the default is Unit
    pub unit: Option<UnitRequest>,
    /// Optional maximum quantity per command, limited to a certain maximum.
    pub max_quantity_per_command: Option<u64>,
    /// Optional SMA code for product identification.
    pub sma_code: Option<String>,
    /// Optional Inventree IPN
    pub inventree_code: Option<String>,
}

impl TryFrom<NewProductRequest> for ActiveModel {
    type Error = ProductRequestError;
    fn try_from(mut value: NewProductRequest) -> Result<Self, Self::Error> {
        Ok(ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            image: Set(value.image),
            name: {
                let name = value.name;
                if name.is_empty() {
                    return Err(Self::Error::NameCannotBeEmpty);
                }
                if name.len() > PRODUCT_NAME_MAX_LENGTH {
                    return Err(Self::Error::NameCannotBeLongerThan(
                        name,
                        PRODUCT_NAME_MAX_LENGTH,
                    ));
                }
                Set(name)
            },
            display_order: Set(0),
            sell_price: {
                let price = value.sell_price;
                if price <= 0.0 {
                    return Err(Self::Error::PriceCannotBeNegativeOrNull(price));
                }

                let price = price.to_string();
                match Decimal::from_str_exact(&price) {
                    Ok(price) => Set(price),
                    Err(err) => {
                        return Err(Self::Error::PriceCannotBeConvertedInDecimal(price, err))
                    }
                }
            },
            sell_price_currency: Set(value.sell_price_currency.into()),
            unit: Set(value.unit.unwrap_or(UnitRequest::Unit).into()),
            max_quantity_per_command: match value.max_quantity_per_command {
                Some(max) => {
                    if max > PRODUCT_MAX_QUANTITY_PER_COMMAND {
                        return Err(Self::Error::MaxQuantityPerCommandCannotBeBiggerThan(
                            max,
                            PRODUCT_MAX_QUANTITY_PER_COMMAND,
                        ));
                    }
                    match max.try_into() {
                        Ok(max) => Set(Some(max)),
                        Err(err) => {
                            return Err(Self::Error::MaxQuantityPerCommandCannotBeConvertedToI16(
                                max, err,
                            ))
                        }
                    }
                }
                None => NotSet,
            },
            sma_code: Set(value.sma_code),
            inventree_code: Set(value.inventree_code),
            purchasable: match value.purchasable {
                Some(purchasable) => {
                    if !purchasable {
                        value.hidden = Some(true);
                        value.disabled = Some(true);
                    }

                    Set(purchasable)
                }
                None => Set(true),
            },
            hidden: match value.hidden {
                Some(hidden) => {
                    if hidden {
                        value.disabled = Some(true);
                    }

                    Set(hidden)
                }
                None => Set(false),
            },
            disabled: Set(value.disabled.unwrap_or(false)),
            created_at: Set(chrono::offset::Local::now().into()),
        })
    }
}

/// Request structure for editing an existing product, allowing optional updates to fields.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditProductRequest {
    /// Optional image URL or path, which can also be set to `None`.
    pub image: Option<Option<String>>,
    /// Optional name of the product with length validation.
    pub name: Option<String>,
    /// Display Order of the product inside of lists, 0 is last + default
    pub display_order: Option<u64>,
    /// Optional price of the product, required to be positive if present.
    pub sell_price: Option<f64>,
    /// Optional price of the product, required to be positive if present.
    pub sell_price_currency: Option<CurrencyRequest>,
    /// If the product is purchasable or if it's just an ingredients
    pub purchasable: Option<bool>,
    /// Represent the unit type of Product, if it's a liquid -> Liter, etc..., the default is Unit
    pub unit: Option<UnitRequest>,
    /// Optional maximum quantity per command with conversion and size limits.
    pub max_quantity_per_command: Option<Option<u64>>,
    /// Optional SMA code for product identification, can be `None` if specified.
    pub sma_code: Option<Option<String>>,
    /// Optional Inventree IPN, can be `None` if specified.
    pub inventree_code: Option<Option<String>>,
    /// Optional field to hide the product.
    pub hidden: Option<bool>,
    /// Optional field to disable or enable the product.
    pub disabled: Option<bool>,
}

impl TryFrom<EditProductRequest> for ActiveModel {
    type Error = ProductRequestError;
    fn try_from(mut value: EditProductRequest) -> Result<Self, Self::Error> {
        Ok(ActiveModel {
            id: NotSet,
            image: match value.image {
                Some(image_opt) => match image_opt {
                    Some(image) => Set(Some(image)),
                    None => Set(None),
                },
                None => NotSet,
            },
            name: match value.name {
                Some(name) => {
                    if name.is_empty() {
                        return Err(Self::Error::NameCannotBeEmpty);
                    }
                    if name.len() > PRODUCT_NAME_MAX_LENGTH {
                        return Err(Self::Error::NameCannotBeLongerThan(
                            name,
                            PRODUCT_NAME_MAX_LENGTH,
                        ));
                    }
                    Set(name)
                }
                None => NotSet,
            },
            display_order: {
                let display_order = value.display_order.unwrap_or(0);
                let display_order_i32 = display_order.try_into().map_err(|err| {
                    Self::Error::DisplayOrderCannotBeConvertedToI32(display_order, err)
                })?;
                Set(display_order_i32)
            },
            sell_price: match value.sell_price {
                Some(price) => {
                    if price <= 0.0 {
                        return Err(Self::Error::PriceCannotBeNegativeOrNull(price));
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
            sell_price_currency: match value.sell_price_currency {
                Some(currency) => Set(currency.into()),
                None => NotSet,
            },
            max_quantity_per_command: match value.max_quantity_per_command {
                Some(max_opt) => match max_opt {
                    Some(max) => {
                        if max > PRODUCT_MAX_QUANTITY_PER_COMMAND {
                            return Err(Self::Error::MaxQuantityPerCommandCannotBeBiggerThan(
                                max,
                                PRODUCT_MAX_QUANTITY_PER_COMMAND,
                            ));
                        }
                        match max.try_into() {
                            Ok(max) => Set(Some(max)),
                            Err(err) => {
                                return Err(
                                    Self::Error::MaxQuantityPerCommandCannotBeConvertedToI16(
                                        max, err,
                                    ),
                                )
                            }
                        }
                    }
                    None => Set(None),
                },
                None => NotSet,
            },
            unit: match value.unit {
                Some(unit) => Set(unit.into()),
                None => NotSet,
            },
            sma_code: match value.sma_code {
                Some(sma_opt) => match sma_opt {
                    Some(sma_code) => Set(Some(sma_code)),
                    None => Set(None),
                },
                None => NotSet,
            },
            inventree_code: match value.inventree_code {
                Some(inventree_opt) => match inventree_opt {
                    Some(inventree_code) => Set(Some(inventree_code)),
                    None => Set(None),
                },
                None => NotSet,
            },
            purchasable: match value.purchasable {
                Some(purchasable) => {
                    if !purchasable {
                        value.hidden = Some(true);
                    }
                    Set(purchasable)
                }
                None => NotSet,
            },
            hidden: match value.hidden {
                Some(hidden) => {
                    if !hidden {
                        value.purchasable = Some(true);
                    }
                    Set(hidden)
                }
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
