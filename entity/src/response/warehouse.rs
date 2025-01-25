//! This module defines the response structures for warehouse-related API responses.

use serde_with::skip_serializing_none;

/// Response structure for a warehouse, including its details.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1",
    "name": "Warehouse Central", 
    "parent": "512731f58-18f1-4c95-8de5-611bde07f4"
}))]
pub struct WarehouseResponse {
    /// Unique identifier for the warehouse.
    id: uuid::Uuid,

    /// Name of the warehouse.
    name: String,
}

/// Response structure for a warehouse ingredient, including its details.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1",
    "product": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288",
    "name": "Warehouse for The Cake",
    "ingredients": [
        { "product": "afd0dac6-70b2-4752-a66f-d79437c53f01", "quantity":  3.0, "disabled": false },
        { "product": "f317ccf1-e196-4bd2-8fb0-106aa05aa899", "quantity":  12.7, "disabled": true },
    ],
    "creation_time": "2024-10-09T17:55:30.795279Z"
}))]
pub struct WarehouseIngredientResponse {
    /// Product use for this ingredient
    product: uuid::Uuid,

    /// Quantity of this ingredient
    quantity: f64,

    /// indicating if the ingredient is disabled.
    disabled: bool,
}

impl From<crate::models::warehouse::Model> for WarehouseResponse {
    /// Constructs a WarehouseResponse from a warehouse model, returning an error if conversion fails.
    fn from(value: crate::models::warehouse::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
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
                "product": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288",
                "name": "Warehouse for The Cake", 
                "ingredients": [
                    { "product": "afd0dac6-70b2-4752-a66f-d79437c53f01", "quantity":  3.0, "disabled": false },
                    { "product": "f317ccf1-e196-4bd2-8fb0-106aa05aa899", "quantity":  12.7, "disabled": true },
                ],
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
