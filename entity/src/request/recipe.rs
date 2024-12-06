//! Defines request structures and conversion logic for creating and editing recipe entities.
//!
//! This module provides request models and error handling for recipe-related operations,
//! including creating new recipes and editing existing ones. It validates request data
//! and ensures that only valid information is converted into the `ActiveModel` format,
//! which is then used for database transactions. Additionally, it defines custom error
//! types to handle validation failures in a structured manner.

use crate::{
    error::impl_bad_request_app_error,
    models::{recipe, recipe_ingredients},
};
use rust_decimal::{Decimal, Error as DecimalError};
use sea_orm::ActiveValue::{NotSet, Set};

/// The maximum allowed length for a recipe name.
/// This constraint ensures that names remain concise and standardized in the database.
pub const RECIPE_NAME_MAX_LENGTH: usize = 32;

/// Errors specific to recipe requests, including validation and conversion errors.
#[derive(Debug, PartialEq, Clone, strum_macros::IntoStaticStr)]
pub enum RecipeRequestError {
    /// Error when the recipe name is empty.
    NameCannotBeEmpty,
    /// Error when the recipe name exceeds the allowed maximum length.
    NameCannotBeLongerThan(String, usize),
    /// Error when the quantity is negative or zero.
    QuantityCannotBeNegativeOrNull(f64),
    /// Error when the quantity cannot be converted into a `Decimal`.
    QuantityCannotBeConvertedInDecimal(String, DecimalError),
    /// Error if the resulting product can't be found in the database.
    ProductCannotBeFound(uuid::Uuid),
    /// Error if the ingredient product can't be found in the database.
    IngredientCannotBeFound(uuid::Uuid),
    /// Error if the recipe can't be found in the database.
    RecipeCannotBeFound(uuid::Uuid),
}
impl std::error::Error for RecipeRequestError {}

impl std::fmt::Display for RecipeRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeRequestError::NameCannotBeEmpty => write!(f, "Name Cannot be Empty"),
            RecipeRequestError::NameCannotBeLongerThan(name, max) => {
                write!(f, "Name \"{name}\" is longer than {max} characters")
            }
            RecipeRequestError::QuantityCannotBeNegativeOrNull(quantity) => {
                write!(f, "Quantity \"{quantity}\" cannot be null or negative")
            }
            RecipeRequestError::QuantityCannotBeConvertedInDecimal(quantity, err) => {
                write!(
                    f,
                    "Quantity \"{quantity}\" cannot be converted in Decimal: {err}"
                )
            }
            RecipeRequestError::ProductCannotBeFound(product) => {
                write!(f, "Product \"{product}\" cannot be found")
            }
            RecipeRequestError::IngredientCannotBeFound(product) => {
                write!(f, "Ingredient \"{product}\" cannot be found")
            }
            RecipeRequestError::RecipeCannotBeFound(recipe) => {
                write!(f, "Recipe \"{recipe}\" cannot be found")
            }
        }
    }
}

impl_bad_request_app_error!(RecipeRequestError);

impl From<RecipeRequestError> for crate::error::ErrorResponse {
    fn from(value: RecipeRequestError) -> Self {
        let kind: &'static str = value.clone().into();
        Self {
            status: 400,
            error: "Bad Request".to_string(),
            kind: kind.to_string(),
            message: value.to_string(),
        }
    }
}

/// Request structure for creating a new recipe, including validation rules.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct NewRecipeRequest {
    /// Name of the recipe, required and validated for length.
    pub name: Option<String>,

    /// Id of the resulting product
    pub product: uuid::Uuid,

    /// List of ingredients
    pub ingredients: Vec<RecipeIngredientRequest>,
}

impl TryFrom<NewRecipeRequest> for recipe::ActiveModel {
    type Error = RecipeRequestError;
    fn try_from(value: NewRecipeRequest) -> Result<Self, Self::Error> {
        Ok(recipe::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            name: match value.name {
                Some(name) => {
                    if name.is_empty() {
                        return Err(Self::Error::NameCannotBeEmpty);
                    }
                    if name.len() > RECIPE_NAME_MAX_LENGTH {
                        return Err(Self::Error::NameCannotBeLongerThan(
                            name,
                            RECIPE_NAME_MAX_LENGTH,
                        ));
                    }
                    Set(Some(name))
                }
                None => Set(None),
            },
            result_product_id: Set(value.product),
            disabled: Set(false),
            created_at: Set(chrono::offset::Local::now().into()),
        })
    }
}

/// Request structure for creating a new recipe ingredient, including validation rules.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct RecipeIngredientRequest {
    /// Product id of the ingredient
    pub product: uuid::Uuid,

    /// Quantity of this product for the recipe
    pub quantity: f64,

    /// Optional field to disable or enable the recipe.
    pub disabled: Option<bool>,
}

impl TryFrom<RecipeIngredientRequest> for recipe_ingredients::ActiveModel {
    type Error = RecipeRequestError;
    fn try_from(value: RecipeIngredientRequest) -> Result<Self, Self::Error> {
        Ok(recipe_ingredients::ActiveModel {
            recipe_id: NotSet,
            ingredient_id: Set(value.product),

            quantity: {
                let quantity = value.quantity;
                if quantity <= 0.0 {
                    return Err(Self::Error::QuantityCannotBeNegativeOrNull(quantity));
                }

                let quantity = quantity.to_string();
                match Decimal::from_str_exact(&quantity) {
                    Ok(quantity) => Set(quantity),
                    Err(err) => {
                        return Err(Self::Error::QuantityCannotBeConvertedInDecimal(
                            quantity, err,
                        ))
                    }
                }
            },

            disabled: match value.disabled {
                Some(disabled) => Set(disabled),
                None => Set(false),
            },
        })
    }
}

/// Request structure for editing an existing recipe, allowing optional updates to fields.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, utoipa::ToSchema)]
pub struct EditRecipeRequest {
    /// Name of the recipe, required and validated for length.
    pub name: Option<Option<String>>,

    /// Id of the resulting product
    pub product: Option<uuid::Uuid>,

    /// List of ingredients
    pub ingredients: Vec<RecipeIngredientRequest>,

    /// Optional field to disable or enable the recipe.
    pub disabled: Option<bool>,
}

impl TryFrom<EditRecipeRequest> for recipe::ActiveModel {
    type Error = RecipeRequestError;
    fn try_from(value: EditRecipeRequest) -> Result<Self, Self::Error> {
        Ok(recipe::ActiveModel {
            id: NotSet,
            name: match value.name {
                Some(name) => match name {
                    Some(name) => {
                        if name.is_empty() {
                            return Err(Self::Error::NameCannotBeEmpty);
                        }
                        if name.len() > RECIPE_NAME_MAX_LENGTH {
                            return Err(Self::Error::NameCannotBeLongerThan(
                                name,
                                RECIPE_NAME_MAX_LENGTH,
                            ));
                        }
                        Set(Some(name))
                    }
                    None => Set(None),
                },
                None => NotSet,
            },
            result_product_id: match value.product {
                Some(product) => Set(product),
                None => NotSet,
            },
            disabled: match value.disabled {
                Some(disabled) => Set(disabled),
                None => NotSet,
            },
            ..Default::default()
        })
    }
}
