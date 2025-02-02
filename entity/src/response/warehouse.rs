//! This module defines the response structures for warehouse-related API responses.

use serde_with::skip_serializing_none;

/// Response structure for a warehouse, including its details.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1",
    "name": "Warehouse Central", 
    "creation_time": "2024-10-09T17:55:30.795279Z"
}))]
pub struct WarehouseResponse {
    /// Unique identifier for the warehouse.
    id: uuid::Uuid,

    /// Name of the warehouse.
    name: String,

    /// The timestamp indicating when the warehouse was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Indicates whether the warehouse is currently disabled.
    pub disabled: bool,
}

impl From<crate::models::warehouse::Model> for WarehouseResponse {
    /// Constructs a WarehouseResponse from a warehouse model, returning an error if conversion fails.
    fn from(value: crate::models::warehouse::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            created_at: value.created_at.into(),
            disabled: value.disabled,
        }
    }
}

/// Response structure for a list of warehouses with pagination details.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!(
    {
        "total_page": 1,
        "current_page": 0,
        "warehouses": [
            {
                "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1",
                "name": "Warehouse Central", 
                "disabled": false,
                "creation_time": "2024-10-09T17:55:30.795279Z"
            }
        ]
    }
))]
pub struct WarehouseListResponse {
    /// Total number of pages available.
    pub total_page: u64,

    /// Current page number.
    pub current_page: u64,

    /// List of warehouses on the current page.
    pub warehouses: Vec<WarehouseResponse>,
}
