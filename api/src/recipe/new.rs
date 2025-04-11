//! This module contains the route handler for creating a new recipe.
//!
//! The handler will be accessible via a POST request to the `/recipe` endpoint.
//! It allows for the creation of new recipe entries in the database.
//! Admin privileges are required to access this route.

use crate::utils::openapi::RECIPE_TAG;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use entity::{
    error::{AppError, ErrorResponse},
    models::{recipe::ActiveModel, recipe_ingredients},
    request::recipe::{NewRecipeRequest, RecipeIngredientRequest, RecipeRequestError},
};
use extractor::profile::admin::Admin;
use service::Connection;

/// Handler for creating a new recipe.
///
/// This function allows an admin to create a new recipe by sending a POST request to the `/recipe` endpoint.
/// The new recipe is validated and stored in the database. The image associated with the recipe is checked in S3 storage.
///
/// - **Admin privileges** are required to access this route.
/// - Returns a `201 Created` status upon successful creation along with the recipe's ID.
///
/// Path: `/recipe`
///
/// - **Request Body:** Expects a `NewRecipe` JSON object.
/// - **Responses:**
///     - 500: Internal server error (likely database related).
///     - 400: Bad request (invalid input data).
///     - 201: Successfully created a new recipe, returns the new recipe's ID as a string.
#[utoipa::path(
    post,
    path = "", 
    tag = RECIPE_TAG,
    request_body(content = NewRecipeRequest, content_type = "application/json"), 
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted.", body = ErrorResponse), 
        (status = 201, description = "Successfully created a new recipe, returns the new recipe's ID as a string.", body = uuid::Uuid)
    )
)]
pub async fn post_new_recipe(
    admin: Admin,
    State(conn): State<Connection>,
    Json(recipe): Json<NewRecipeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let recipe_model: ActiveModel = recipe.clone().try_into()?;

    // Verifiy that every product exist before mutating anything
    let result_product = service::Query::find_product_by_id(&conn, recipe.product)
        .await?
        .ok_or(RecipeRequestError::ProductCannotBeFound(recipe.product))?;

    if result_product.unit != entity::models::sea_orm_active_enums::Unit::Unit {
        return Err(RecipeRequestError::ResultingProductIsNotUnit(
            result_product.id,
            result_product.unit.into(),
        )
        .into());
    }

    for ingredient in recipe.ingredients.clone() {
        TryInto::<recipe_ingredients::ActiveModel>::try_into(ingredient.clone())?;
        if ingredient.product == recipe.product {
            return Err(
                RecipeRequestError::IngredientCannotBeResultingProduct(ingredient.product).into(),
            );
        }

        service::Query::find_product_by_id(&conn, ingredient.product)
            .await?
            .ok_or(RecipeRequestError::IngredientCannotBeFound(
                ingredient.product,
            ))?;
    }
    //

    let result = service::Mutation::create_recipe(&conn, recipe_model).await?;
    let id = result.id;

    for ingredient in merge_ingredients(recipe.ingredients) {
        let ingredient_model: recipe_ingredients::ActiveModel = ingredient.clone().try_into()?;

        service::Mutation::add_recipe_ingredient(&conn, id, ingredient.product, ingredient_model)
            .await?;
    }

    log::info!(
        "{admin} added a new recipe {} for {} - {:?}",
        id,
        result.result_product_id,
        result
    );

    Ok((StatusCode::CREATED, id.to_string()).into_response())
}

/// Merges a list of `RecipeIngredientRequest` items by combining their quantities
/// and handling their `disabled` states. Ingredients with the same `product` are merged.
fn merge_ingredients<I: IntoIterator<Item = RecipeIngredientRequest>>(
    ingredients: I,
) -> Vec<RecipeIngredientRequest> {
    ingredients
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, ingredient| {
            acc.entry(ingredient.product)
                .and_modify(|e: &mut RecipeIngredientRequest| {
                    e.quantity += ingredient.quantity;
                    e.disabled =
                        Some(e.disabled.unwrap_or(true) && ingredient.disabled.unwrap_or(true));
                })
                .or_insert(ingredient);
            acc
        })
        .into_values()
        .collect()
}
