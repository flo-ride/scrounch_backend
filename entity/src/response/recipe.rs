//! This module defines the response structures for recipe-related API responses.
//! The module also defines the `RecipeResponseError` enum for error handling during
//! recipe response construction, particularly for price and quantity conversions.

use crate::error::impl_from_error_to_string;
use rust_decimal::{Decimal, Error as DecimalError};
use serde_with::skip_serializing_none;

/// Enum representing errors that can occur during recipe response construction.
#[derive(Debug, PartialEq, Clone)]
pub enum RecipeResponseError {
    /// Error indicating that a price cannot be converted from Decimal.
    QuantityCannotBeConverted(Decimal, DecimalError),
}
impl std::error::Error for RecipeResponseError {}

impl std::fmt::Display for RecipeResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QuantityCannotBeConverted(price, err) => {
                write!(f, "Price \"{price}\" cannot be converted in : {err}")
            }
        }
    }
}
impl_from_error_to_string!(RecipeResponseError, InternalError);

/// Response structure for a recipe, including its details.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1",
    "product": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288",
    "name": "Recipe for The Cake", 
    "ingredients": [
        { "product": "afd0dac6-70b2-4752-a66f-d79437c53f01", "quantity":  3.0, "disabled": false },
        { "product": "f317ccf1-e196-4bd2-8fb0-106aa05aa899", "quantity":  12.7, "disabled": true },
    ],
    "creation_time": "2024-10-09T17:55:30.795279Z"
}))]
pub struct RecipeResponse {
    /// Unique identifier for the recipe.
    id: uuid::Uuid,

    /// Name of the recipe.
    name: Option<String>,

    /// Resulting product of this recipe
    product: uuid::Uuid,

    /// List of ingredients
    ingredients: Vec<RecipeIngredientResponse>,

    /// indicating if the recipe is disabled.
    disabled: bool,

    /// The timestamp indicating when the recipe was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Response structure for a recipe ingredient, including its details.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!({
    "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1",
    "product": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288",
    "name": "Recipe for The Cake",
    "ingredients": [
        { "product": "afd0dac6-70b2-4752-a66f-d79437c53f01", "quantity":  3.0, "disabled": false },
        { "product": "f317ccf1-e196-4bd2-8fb0-106aa05aa899", "quantity":  12.7, "disabled": true },
    ],
    "creation_time": "2024-10-09T17:55:30.795279Z"
}))]
pub struct RecipeIngredientResponse {
    /// Product use for this ingredient
    product: uuid::Uuid,

    /// Quantity of this ingredient
    quantity: f64,

    /// indicating if the ingredient is disabled.
    disabled: bool,
}

impl TryFrom<crate::models::recipe::Model> for RecipeResponse {
    type Error = RecipeResponseError;

    /// Constructs a RecipeResponse from a recipe model, returning an error if conversion fails.
    fn try_from(value: crate::models::recipe::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name,
            product: value.result_product_id,
            ingredients: vec![],
            disabled: value.disabled,
            created_at: value.created_at.into(),
        })
    }
}

impl TryFrom<crate::models::recipe_ingredients::Model> for RecipeIngredientResponse {
    type Error = RecipeResponseError;

    /// Constructs a RecipeResponse from a recipe model, returning an error if conversion fails.
    fn try_from(value: crate::models::recipe_ingredients::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            product: value.ingredient_id,
            quantity: value
                .quantity
                .try_into()
                .map_err(|err| Self::Error::QuantityCannotBeConverted(value.quantity, err))?,
            disabled: value.disabled,
        })
    }
}

impl
    TryFrom<(
        crate::models::recipe::Model,
        Vec<crate::models::recipe_ingredients::Model>,
    )> for RecipeResponse
{
    type Error = RecipeResponseError;

    /// Constructs a RecipeResponse from a recipe model, returning an error if conversion fails.
    fn try_from(
        (recipe, ingredients): (
            crate::models::recipe::Model,
            Vec<crate::models::recipe_ingredients::Model>,
        ),
    ) -> Result<Self, Self::Error> {
        let mut ingredients = ingredients
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, Self::Error>>()?;
        ingredients.sort_by(
            |a: &RecipeIngredientResponse, b: &RecipeIngredientResponse| {
                a.quantity
                    .partial_cmp(&b.quantity)
                    .unwrap_or(std::cmp::Ordering::Equal)
            },
        );

        Ok(Self {
            id: recipe.id,
            name: recipe.name,
            product: recipe.result_product_id,
            ingredients,
            created_at: recipe.created_at.into(),
            disabled: recipe.disabled,
        })
    }
}

/// Response structure for a list of recipes with pagination details.
#[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)]
#[schema(example = json!(
    {
        "total_page": 1,
        "current_page": 0,
        "recipes": [
            {
                "id": "1a731f58-18f1-4c95-8de5-611bde07f4f1",
                "product": "2fa4c8d3-fd93-4066-a7f3-68a35ab72288",
                "name": "Recipe for The Cake", 
                "ingredients": [
                    { "product": "afd0dac6-70b2-4752-a66f-d79437c53f01", "quantity":  3.0, "disabled": false },
                    { "product": "f317ccf1-e196-4bd2-8fb0-106aa05aa899", "quantity":  12.7, "disabled": true },
                ],
                "creation_time": "2024-10-09T17:55:30.795279Z"
            }
        ]
    }
))]
pub struct RecipeListResponse {
    /// Total number of pages available.
    pub total_page: u64,

    /// Current page number.
    pub current_page: u64,

    /// List of recipes on the current page.
    pub recipes: Vec<RecipeResponse>,
}
