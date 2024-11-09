//! This module defines the response structures for product-related API responses.
//! The module also defines the `ProductResponseError` enum for error handling during
//! product response construction, particularly for price and quantity conversions.

use super::r#enum::CurrencyResponse;
use crate::error::impl_from_error_to_string;
use rust_decimal::{Decimal, Error as DecimalError};
use serde_with::skip_serializing_none;
use std::num::TryFromIntError;

/// Enum representing errors that can occur during product response construction.
#[derive(Debug, PartialEq, Clone)]
pub enum ProductResponseError {
    /// Error indicating that a price cannot be converted from Decimal.
    PriceCannotBeConverted(Decimal, DecimalError),
    /// Error indicating that the maximum quantity per command cannot be converted from i16.
    MaxPerCommandCannotBeConverted(i16, TryFromIntError),
}
impl std::error::Error for ProductResponseError {}

impl std::fmt::Display for ProductResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductResponseError::PriceCannotBeConverted(price, err) => {
                write!(f, "Price \"{price}\" cannot be converted in : {err}")
            }
            ProductResponseError::MaxPerCommandCannotBeConverted(max_quantity_per_command, err) => {
                write!(f, "Max Quantity Per Command \"{max_quantity_per_command}\" cannot be converted {err}")
            }
        }
    }
}
impl_from_error_to_string!(ProductResponseError, InternalError);

/// Response structure for a product, including its details.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1", 
    "image": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288_water.png", 
    "name": "water", 
    "price": 0.80, 
    "creation_time": "2024-10-09T17:55:30.795279Z" 
}))]
pub struct ProductResponse {
    /// Optional image associated with the product.
    image: Option<String>,

    /// Unique identifier for the product.
    id: uuid::Uuid,

    /// Name of the product.
    name: String,

    /// Price of the product.
    price: f64,

    /// Currency of the product price.
    currency: CurrencyResponse,

    /// Optional maximum quantity allowed per command.
    max_quantity_per_command: Option<u64>,

    /// Optional SMA code associated with the product.
    sma_code: Option<String>,

    /// Creation timestamp of the product.
    created_at: chrono::DateTime<chrono::Utc>,

    /// Optional flag indicating if the product is disabled.
    disabled: Option<bool>,
}

impl TryFrom<crate::models::product::Model> for ProductResponse {
    type Error = ProductResponseError;

    /// Constructs a ProductResponse from a product model, returning an error if conversion fails.
    fn try_from(value: crate::models::product::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            image: value.image,
            id: value.id,
            name: value.name,
            price: value
                .price
                .try_into()
                .map_err(|err| Self::Error::PriceCannotBeConverted(value.price, err))?,
            currency: value.price_currency.into(),
            max_quantity_per_command: match value.max_quantity_per_command {
                Some(x) => Some(
                    x.try_into()
                        .map_err(|err| Self::Error::MaxPerCommandCannotBeConverted(x, err))?,
                ),
                None => None,
            },
            sma_code: value.sma_code,
            created_at: value.created_at.into(),
            disabled: match value.disabled {
                true => Some(true),
                false => None,
            },
        })
    }
}

/// Response structure for edited product details.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1", 
    "image": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288_water.png", 
    "name": "water", 
    "price": 0.80, 
    "creation_time": "2024-10-09T17:55:30.795279Z" 
}))]
pub struct EditedProductResponse {
    /// Unique identifier for the product.
    pub id: uuid::Uuid,

    /// Optional image associated with the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Optional name of the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional price of the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Optional maximum quantity allowed per command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity_per_command: Option<u64>,

    /// Optional SMA code associated with the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sma_code: Option<String>,

    /// Optional flag indicating if the product is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// Response structure for a list of products with pagination details.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!(
    {
        "total_page": 1, 
        "current_page": 0, 
        "products": [
            {
                "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1", 
                "image": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288_water.png", 
                "name": "water", 
                "price": 0.80, 
                "creation_time": "2024-10-09T17:55:30.795279Z" 
            },
            {
                "id": "0a7e6dd2-2c98-44b1-9cd3-0d8a3d7666b3", 
                "image": "377265f4-1aad-4b57-a6f2-4bb6387184c2_tea.png", 
                "name": "tea", 
                "price": 1.52, 
                "creation_time": "2024-10-09T18:32:10.795279Z" 
            }
        ]
    }
))]
pub struct ProductListResponse {
    /// Total number of pages available.
    pub total_page: u64,

    /// Current page number.
    pub current_page: u64,

    /// List of products on the current page.
    pub products: Vec<ProductResponse>,
}
