//! This module defines the `SmaResponse` structure used for handling responses related to SMA product imports or synchronization.
//! This structure is useful for API responses where products need to be classified by their synchronization status with an external system.

use super::product::{EditedProductResponse, ProductResponse};

/// Represents the response structure for synchronizing products with the SMA system,
/// categorizing products based on their synchronization status.
/// This structure is intended for use in API responses where the status of products
/// in relation to the SMA system needs to be clearly distinguished.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct SmaResponse {
    /// A list of product IDs that have not been modified during the synchronization process.
    pub unchanged: Vec<uuid::Uuid>,

    /// A list of products that were updated, represented by `EditedProductResponse`.
    pub changed: Vec<EditedProductResponse>,

    /// A list of newly created products, represented by `ProductResponse`.
    pub created: Vec<ProductResponse>,
}
