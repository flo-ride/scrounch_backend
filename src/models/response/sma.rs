use crate::{error::AppError, models::utils::sma::SmaChange};

use super::product::{EditedProductResponse, ProductResponse};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SmaResponse {
    pub unchanged: Vec<uuid::Uuid>,
    pub changed: Vec<EditedProductResponse>,
    pub created: Vec<ProductResponse>,
}

impl TryFrom<Vec<SmaChange>> for SmaResponse {
    type Error = AppError;

    fn try_from(value: Vec<SmaChange>) -> Result<Self, Self::Error> {
        let iter = value.into_iter();
        Ok(Self {
            unchanged: iter
                .clone()
                .filter_map(|x| match x {
                    SmaChange::Unchanged(x) => Some(x),
                    _ => None,
                })
                .map(|x| x.id)
                .collect(),
            changed: vec![], // TODO: Changed
            created: iter
                .filter_map(|x| match x {
                    SmaChange::Created(x) => Some(x),
                    _ => None,
                })
                .map(TryInto::<ProductResponse>::try_into)
                .collect::<Result<_, AppError>>()?,
        })
    }
}
