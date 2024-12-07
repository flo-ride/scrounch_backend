#[cfg(feature = "cache")]
use crate::r#macro::{cache_get, cache_mget, cache_mset, cache_set};
use crate::{query::Query, Connection};
use ::entity::models::{
    prelude::{Recipe, RecipeIngredients},
    recipe, recipe_ingredients,
};
use sea_orm::*;

impl Query {
    pub async fn find_recipe_by_id(
        conn: &Connection,
        id: uuid::Uuid,
    ) -> Result<Option<(recipe::Model, Vec<recipe_ingredients::Model>)>, DbErr> {
        #[cfg(feature = "cache")]
        cache_get!(
            conn,
            format!("recipe:{id}"),
            (recipe::Model, Vec<recipe_ingredients::Model>)
        );

        let result: Option<(recipe::Model, Vec<recipe_ingredients::Model>)> =
            Recipe::find_by_id(id)
                .find_with_related(RecipeIngredients)
                .all(&conn.db_connection)
                .await?
                .first()
                .cloned();

        #[cfg(feature = "cache")]
        if let Some(model) = &result {
            cache_set!(conn, format!("recipe:{id}"), model, 60 * 60 * 3);
        }

        Ok(result)
    }

    pub async fn list_recipes_with_condition<
        F: sea_query::IntoCondition + std::fmt::Debug + Clone,
        S: IntoIterator<Item = (impl IntoSimpleExpr, Order)> + std::fmt::Debug + Clone,
        A: Into<u64> + Copy,
        P: Into<u64> + Copy,
    >(
        conn: &Connection,
        filter: F,
        sort: S,
        page: A,
        per_page: P,
    ) -> Result<Vec<(recipe::Model, Vec<recipe_ingredients::Model>)>, DbErr> {
        #[cfg(feature = "cache")]
        cache_mget!(
            conn,
            format!(
                "recipes:{filter:?}-{sort:?}-{}/{}",
                page.into(),
                per_page.into()
            ),
            (recipe::Model, Vec<recipe_ingredients::Model>)
        );

        let mut query = Recipe::find()
            .find_with_related(RecipeIngredients)
            .filter(filter.clone());
        for (column, order) in sort.clone() {
            query = query.order_by_with_nulls(column, order, sea_query::NullOrdering::Last);
        }
        let query = query
            .offset(page.into() * per_page.into())
            .limit(per_page.into());

        let result = query.all(&conn.db_connection).await?;

        #[cfg(feature = "cache")]
        cache_mset!(
            conn,
            format!(
                "recipes:{filter:?}-{sort:?}-{}/{}",
                page.into(),
                per_page.into()
            ),
            result,
            60 * 60 * 3,
            |x: &(recipe::Model, Vec<recipe_ingredients::Model>)| format!("recipe:{}", x.0.id)
        );

        Ok(result)
    }

    pub async fn count_recipes_with_condition<F: sea_query::IntoCondition>(
        conn: &Connection,
        filter: F,
    ) -> Result<u64, DbErr> {
        Recipe::find()
            .filter(filter)
            .count(&conn.db_connection)
            .await
    }
}
