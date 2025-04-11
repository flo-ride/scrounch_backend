//! Route for editing an existing recipe in the store.

use crate::utils::openapi::RECIPE_TAG;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::{
    error::AppError,
    models::{recipe, recipe_ingredients},
    request::recipe::{EditRecipeRequest, RecipeIngredientRequest, RecipeRequestError},
};
use extractor::profile::admin::Admin;
use sea_orm::ActiveValue::Set;
use service::Connection;

/// Edit an existing recipe by ID in the store.
///
/// The admin can change attributes such as the name, price, quantity, or image of the recipe.
/// If the recipe image is changed, the old image will be deleted from S3 storage.
///
/// Returns an error if the recipe doesn't exist, if there is a validation issue, or if a database or S3 operation fails.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = RECIPE_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "Recipe database id to edit recipe for"),
    ),
    request_body(content = EditRecipeRequest, content_type = "application/json"), 
    responses(
        (status = 500, description = "An internal error occured, probably database related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The recipe is correctly edited")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn edit_recipe(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
    Json(edit_recipe): Json<EditRecipeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_recipe_by_id(&conn, id).await?;

    match result {
        Some(existing_recipe) => {
            let edit_recipe_model: recipe::ActiveModel = edit_recipe.clone().try_into()?;

            // Verifiy that every product exist before mutating anything
            if let Set(product) = edit_recipe_model.result_product_id {
                if existing_recipe.0.result_product_id != product {
                    service::Query::find_product_by_id(&conn, product)
                        .await?
                        .ok_or(RecipeRequestError::ProductCannotBeFound(product))?;
                }
            }

            for ingredient in edit_recipe.ingredients.clone() {
                TryInto::<recipe_ingredients::ActiveModel>::try_into(ingredient.clone())?;

                service::Query::find_product_by_id(&conn, ingredient.product)
                    .await?
                    .ok_or(RecipeRequestError::IngredientCannotBeFound(
                        ingredient.product,
                    ))?;
            }
            //

            let result = service::Mutation::update_recipe(&conn, id, edit_recipe_model).await?;

            let edit_ingredients = merge_ingredients(edit_recipe.ingredients);

            let new_ingredients_id: std::collections::HashSet<_> =
                edit_ingredients.iter().map(|x| x.product).collect();
            let existing_ingredients_id: std::collections::HashSet<_> =
                existing_recipe.1.iter().map(|x| x.ingredient_id).collect();

            let edit_products: Vec<_> = new_ingredients_id
                .intersection(&existing_ingredients_id)
                .cloned()
                .filter_map(|x| {
                    edit_ingredients
                        .clone()
                        .into_iter()
                        .find(|y| y.product == x)
                })
                .collect();

            for ingredient in &edit_products {
                let model: recipe_ingredients::ActiveModel = ingredient.clone().try_into()?;
                service::Mutation::update_recipe_ingredient(&conn, id, ingredient.product, model)
                    .await?;
            }

            let new_products: Vec<_> = new_ingredients_id
                .difference(&existing_ingredients_id)
                .cloned()
                .filter_map(|x| {
                    edit_ingredients
                        .clone()
                        .into_iter()
                        .find(|y| y.product == x)
                })
                .collect();

            for ingredient in &new_products {
                let model: recipe_ingredients::ActiveModel = ingredient.clone().try_into()?;
                service::Mutation::add_recipe_ingredient(&conn, id, ingredient.product, model)
                    .await?;
            }

            let delete_ids: Vec<_> = existing_ingredients_id
                .difference(&new_ingredients_id)
                .cloned()
                .collect();

            for ingredient_id in &delete_ids {
                service::Mutation::delete_recipe_ingredient(&conn, id, *ingredient_id).await?;
            }

            log::info!(
                "{admin} successfully edited recipe \"{}\" - {:?}",
                id,
                result
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The recipe with id: {id} doesn't exist"
        ))),
    }
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
