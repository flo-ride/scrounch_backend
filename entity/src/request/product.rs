use std::num::TryFromIntError;

use rust_decimal::{Decimal, Error as DecimalError};
use sea_orm::ActiveValue::{NotSet, Set};

use crate::models::product::ActiveModel;

pub const PRODUCT_NAME_MAX_LENGTH: usize = 32;
pub const PRODUCT_MAX_QUANTITY_PER_COMMAND: u64 = 10;

#[derive(Debug, PartialEq, Clone)]
pub enum ProductRequestError {
    NameCannotBeEmpty,
    NameCannotBeLongerThan(String, usize),
    PriceCannotBeNegativeOrNull(f64),
    PriceCannotBeConvertedInDecimal(String, DecimalError),
    MaxQuantityPerCommandCannotBeBiggerThan(u64, u64),
    MaxQuantityPerCommandCannotBeConvertedToI16(u64, TryFromIntError),
    ImageDoesNotExist(String),
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
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewProductRequest {
    pub image: Option<String>,
    pub name: String,
    pub price: f64,
    pub max_quantity_per_command: Option<u64>,
    pub sma_code: Option<String>,
}

impl TryFrom<NewProductRequest> for ActiveModel {
    type Error = ProductRequestError;
    fn try_from(value: NewProductRequest) -> Result<Self, Self::Error> {
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
            price: {
                let price = value.price;
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
            disabled: Set(false),
            creation_time: Set(chrono::offset::Local::now().into()),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditProductRequest {
    pub image: Option<Option<String>>,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub max_quantity_per_command: Option<Option<u64>>,
    pub sma_code: Option<Option<String>>,
    pub disabled: Option<bool>,
}

impl TryFrom<EditProductRequest> for ActiveModel {
    type Error = ProductRequestError;
    fn try_from(value: EditProductRequest) -> Result<Self, Self::Error> {
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
            price: match value.price {
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
            sma_code: match value.sma_code {
                Some(sma_opt) => match sma_opt {
                    Some(sma_code) => Set(Some(sma_code)),
                    None => Set(None),
                },
                None => NotSet,
            },
            disabled: match value.disabled {
                Some(disabled) => Set(disabled),
                None => NotSet,
            },
            creation_time: NotSet,
        })
    }
}
