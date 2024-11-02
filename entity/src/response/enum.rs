//! This module defines the response format for currency types in the API,
//! facilitating serialization and deserialization to/from lowercase strings.

use crate::models::sea_orm_active_enums::Currency;

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
