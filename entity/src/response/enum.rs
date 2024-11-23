//! This module defines the response format for currency types in the API,
//! facilitating serialization and deserialization to/from lowercase strings.

use crate::models::sea_orm_active_enums::{Currency, Unit};

/// Represents the response format for currency types in the API,
/// enabling serialization and deserialization to/from lowercase strings.
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyResponse {
    /// Represents the Euro currency.
    Euro,

    /// Represents the Epicoin currency.
    Epicoin,
}

impl From<Currency> for CurrencyResponse {
    fn from(value: Currency) -> Self {
        match value {
            Currency::Euro => Self::Euro,
            Currency::Epicoin => Self::Epicoin,
        }
    }
}

/// Represents the different type of Unit an Product or Else can have
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UnitResponse {
    /// Represents a single unit or piece (e.g., an item).
    Unit,
    /// Represents weight in grams (as the base unit for mass).
    Gram,
    /// Represents volume in liters (as the base unit for volume).
    Liter,
    /// Represents length in meters (as the base unit for distance).
    Meter,
}

impl From<Unit> for UnitResponse {
    fn from(value: Unit) -> Self {
        match value {
            Unit::Unit => Self::Unit,
            Unit::Gram => Self::Gram,
            Unit::Liter => Self::Liter,
            Unit::Meter => Self::Meter,
        }
    }
}
