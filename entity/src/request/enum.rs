//! This module defines the request format for currency types in the API,
//! facilitating serialization and deserialization to/from lowercase strings.

use crate::models::sea_orm_active_enums::{Currency, Unit};

/// Represents the request format for currency types in the API,
/// enabling serialization and deserialization to/from lowercase strings.
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyRequest {
    /// Represents the Euro currency.
    Euro,

    /// Represents the Epicoin currency.
    Epicoin,
}

impl From<CurrencyRequest> for Currency {
    fn from(value: CurrencyRequest) -> Self {
        match value {
            CurrencyRequest::Euro => Self::Euro,
            CurrencyRequest::Epicoin => Self::Epicoin,
        }
    }
}

impl From<CurrencyRequest> for Option<Currency> {
    fn from(value: CurrencyRequest) -> Self {
        Some(value.into())
    }
}

/// Represents the different type of Unit an Product or Else can have
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UnitRequest {
    /// Represents a single unit or piece (e.g., an item).
    Unit,
    /// Represents weight in grams (as the base unit for mass).
    Gram,
    /// Represents volume in liters (as the base unit for volume).
    Liter,
    /// Represents length in meters (as the base unit for distance).
    Meter,
}

impl From<UnitRequest> for Unit {
    fn from(value: UnitRequest) -> Self {
        match value {
            UnitRequest::Unit => Self::Unit,
            UnitRequest::Gram => Self::Gram,
            UnitRequest::Liter => Self::Liter,
            UnitRequest::Meter => Self::Meter,
        }
    }
}
