use chrono::Utc;

use crate::error::AppError;

#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({ "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1", "image": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288_water.png", "name": "water", "price": 0.80, "quantity": 27, "creation_time": "2024-10-09T17:55:30.795279Z" }))]
pub struct ProductResponse {
    image: String,

    id: uuid::Uuid,

    name: String,

    price: f64,

    quantity: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_quantity_per_command: Option<u64>,

    creation_time: chrono::DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
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

            quantity: value.quantity.try_into().map_err(|err| {
                AppError::Unknow(format!(
                    "Cannot convert quantity from i16 to u64: {} - {err}",
                    value.quantity
                ))
            })?,
            max_quantity_per_command: match value.max_quantity_per_command {
                Some(x) => Some(x.try_into().map_err(|err| {
                    AppError::Unknow(format!(
                        "Cannot convert Max Quantity per Command from i16 to u64: {x} - {err}"
                    ))
                })?),
                None => None,
            },
            creation_time: value.creation_time.into(),
            disabled: match value.disabled {
                true => Some(true),
                false => None,
            },
        })
    }
}
