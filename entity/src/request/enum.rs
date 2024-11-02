//! This module defines the request format for currency types in the API,
//! facilitating serialization and deserialization to/from lowercase strings.

use crate::models::sea_orm_active_enums::Currency;

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
