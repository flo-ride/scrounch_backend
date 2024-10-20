#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewProduct {
    pub image: Option<String>,
    pub name: String,
    pub price: f64,
    pub quantity: u64,
    pub max_quantity_per_command: Option<u64>,
    pub sma_code: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditProduct {
    pub image: Option<String>,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub quantity: Option<u64>,
    pub max_quantity_per_command: Option<u64>,
    pub sma_code: Option<String>,
    pub disabled: Option<bool>,
}
