use chrono::Utc;

use crate::error::AppError;

#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1", "image": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288_water.png", "name": "water", "price": 0.80, "quantity": 27, "creation_time": "2024-10-09T17:55:30.795279Z" }))]
pub struct ProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,

    id: uuid::Uuid,

    name: String,

    price: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_quantity_per_command: Option<u64>,

    creation_time: chrono::DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sma_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1", "image": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288_water.png", "name": "water", "price": 0.80, "quantity": 27, "creation_time": "2024-10-09T17:55:30.795279Z" }))]
pub struct EditedProductResponse {
    pub id: uuid::Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity_per_command: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sma_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl TryFrom<entity::product::Model> for ProductResponse {
    type Error = AppError;

    fn try_from(value: entity::product::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            image: value.image,
            id: value.id,
            name: value.name,
            price: value.price.try_into().map_err(|err| {
                AppError::Unknow(format!("Cannot convert price {} - {err}", value.price))
            })?,

            quantity: match value.quantity {
                Some(x) => Some(x.try_into().map_err(|err| {
                    AppError::Unknow(format!(
                        "Cannot convert quantity from i16 to u64: {x} - {err}",
                    ))
                })?),
                None => None,
            },
            max_quantity_per_command: match value.max_quantity_per_command {
                Some(x) => Some(x.try_into().map_err(|err| {
                    AppError::Unknow(format!(
                        "Cannot convert Max Quantity per Command from i16 to u64: {x} - {err}"
                    ))
                })?),
                None => None,
            },
            sma_code: value.sma_code,
            creation_time: value.creation_time.into(),
            disabled: match value.disabled {
                true => Some(true),
                false => None,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!(
    {
        "total_page": 1, 
        "current_page": 0, 
        "products": [
            { "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1", "image": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288_water.png", "name": "water", "price": 0.80, "quantity": 27, "creation_time": "2024-10-09T17:55:30.795279Z" }, 
            { "id": "0a7e6dd2-2c98-44b1-9cd3-0d8a3d7666b3", "image": "377265f4-1aad-4b57-a6f2-4bb6387184c2_tea.png", "name": "tea", "price": 1.52, "quantity": 42, "creation_time": "2024-10-09T18:32:10.795279Z" }
        ]
    }
))]
pub struct ProductListResponse {
    pub total_page: u64,
    pub current_page: u64,
    pub products: Vec<ProductResponse>,
}
