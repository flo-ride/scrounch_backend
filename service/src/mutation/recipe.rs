#[cfg(feature = "cache")]
use crate::r#macro::{cache_del, cache_mdel};
use crate::{Connection, mutation::Mutation};
use ::entity::models::{prelude::*, recipe, recipe_ingredients};
use sea_orm::*;

impl Mutation {
    pub async fn create_recipe<M: IntoActiveModel<recipe::ActiveModel>>(
        conn: &Connection,
        form_data: M,
    ) -> Result<recipe::Model, DbErr> {
        let form_data = form_data.into_active_model();
        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(_model) = &result {
            cache_mdel!(conn, "recipes");
        }

        result
    }

    pub async fn update_recipe<M: IntoActiveModel<recipe::ActiveModel>>(
        conn: &Connection,
        id: uuid::Uuid,
        form_data: M,
    ) -> Result<recipe::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.id = Set(id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(_model) = &result {
            // Delete key instead of editing it
            cache_del!(conn, format!("recipe:{id}"));
            cache_mdel!(conn, "recipes");
        }

        result
    }

    pub async fn delete_recipe(conn: &Connection, id: uuid::Uuid) -> Result<DeleteResult, DbErr> {
        let recipe: recipe::ActiveModel = Recipe::find_by_id(id)
            .one(&conn.db_connection)
            .await?
            .ok_or(DbErr::Custom(format!("Cannot find recipe: \"{id}\"")))
            .map(Into::into)?;

        let result = recipe.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(conn, format!("recipe:{id}"));
            cache_mdel!(conn, "recipes");
        }

        result
    }

    pub async fn delete_all_recipes(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Recipe::delete_many().exec(db).await
    }

    pub async fn add_recipe_ingredient<M: IntoActiveModel<recipe_ingredients::ActiveModel>>(
        conn: &Connection,
        recipe_id: uuid::Uuid,
        product_id: uuid::Uuid,
        form_data: M,
    ) -> Result<recipe_ingredients::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.recipe_id = Set(recipe_id);
        form_data.ingredient_id = Set(product_id);

        let result = form_data.insert(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(_model) = &result {
            // Delete key instead of editing it
            cache_del!(conn, format!("recipe:{recipe_id}"));
            cache_mdel!(conn, "recipes");
        }

        result
    }

    pub async fn update_recipe_ingredient<M: IntoActiveModel<recipe_ingredients::ActiveModel>>(
        conn: &Connection,
        recipe_id: uuid::Uuid,
        product_id: uuid::Uuid,
        form_data: M,
    ) -> Result<recipe_ingredients::Model, DbErr> {
        let mut form_data = form_data.into_active_model();
        form_data.recipe_id = Set(recipe_id);
        form_data.ingredient_id = Set(product_id);

        let result = form_data.update(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if let Ok(_model) = &result {
            // Delete key instead of editing it
            cache_del!(conn, format!("recipe:{recipe_id}"));
            cache_mdel!(conn, "recipes");
        }

        result
    }

    pub async fn delete_recipe_ingredient(
        conn: &Connection,
        recipe_id: uuid::Uuid,
        product_id: uuid::Uuid,
    ) -> Result<DeleteResult, DbErr> {
        let recipe: recipe_ingredients::ActiveModel =
            RecipeIngredients::find_by_id((recipe_id, product_id))
                .one(&conn.db_connection)
                .await?
                .ok_or(DbErr::Custom(format!(
                    "Cannot find recipe ingredient: (\"{recipe_id}\", \"{product_id}\")"
                )))
                .map(Into::into)?;

        let result = recipe.delete(&conn.db_connection).await;

        #[cfg(feature = "cache")]
        if result.is_ok() {
            cache_del!(conn, format!("recipe:{recipe_id}"));
            cache_mdel!(conn, "recipes");
        }

        result
    }
}
