use entity::product::Model as Product;
use serde::Deserialize;
use serde_json::Value;

use crate::models::response::product::EditedProductResponse;

#[derive(Debug, Clone, PartialEq)]
pub enum SmaChange {
    Unchanged(Product),
    Edited(EditedProductResponse),
    Created(Product),
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct SmaChangeTypeMatrix {
    #[serde(default)]
    pub name: bool,
    #[serde(default)]
    pub price: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SmaProducts {
    pub data: Vec<SmaProduct>,
    pub limit: Value,
    pub start: Value,
    pub total: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SmaProduct {
    pub category: SmCategory,
    pub code: String,
    pub id: String,
    pub image_url: Option<String>,
    pub name: String,
    pub net_price: String,
    pub price: String,
    pub slug: String,
    pub tax_method: String,
    pub tax_rate: SmaTaxRate,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unit: SmaUnit,
    pub unit_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SmCategory {
    pub id: String,
    pub code: String,
    pub name: String,
    pub image: Option<String>,
    pub parent_id: String,
    pub slug: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SmaTaxRate {
    pub id: String,
    pub name: String,
    pub code: String,
    pub rate: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SmaUnit {
    pub id: String,
    pub code: String,
    pub name: String,
    pub base_unit: Option<String>,
    pub operator: Option<String>,
    pub unit_value: Option<String>,
    pub operation_value: Option<String>,
}
