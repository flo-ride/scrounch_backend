//! This module contains the route handler for retrieving recipe information.

use crate::utils::openapi::RECIPE_TAG;
use axum::{
    extract::{Path, State},
    Json,
};
use axum_extra::extract::Query;
use entity::{
    error::AppError,
    models::recipe::{RecipeFilterQuery, RecipeSortQuery},
    response::recipe::{RecipeListResponse, RecipeResponse, RecipeResponseError},
};
use extractor::{profile::admin::Admin, query::Pagination};
use service::Connection;

/// Handles the request to fetch a recipe by its unique identifier.
///
/// - **Path Parameters**:  
///   `id` (UUID): The database ID of the recipe to retrieve.
///
/// - **Response Codes**:  
///   - `200 OK`: The recipe was successfully retrieved.
///   - `404 Not Found`: The recipe doesn't exist, or is disabled and the requester is not an admin.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   If the recipe is hidden, only an admin can retrieve it.
#[utoipa::path(get, path = "/{id}", 
    tag = RECIPE_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "The database ID of the recipe to retrieve."),
    ),
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 404, description = "The recipe doesn't exist, or is disabled and the requester is not an admin."), 
        (status = 200, description = "The recipe was successfully retrieved.", body = RecipeResponse)
    ),
    security(
        (),
        ("axum-oidc" = [])
    )
)]
pub async fn get_recipe(
    _admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
) -> Result<Json<RecipeResponse>, AppError> {
    let result = service::Query::find_recipe_by_id(&conn, id).await?;

    match result {
        Some(recipe) => Ok(Json(recipe.try_into()?)),
        None => Err(AppError::NotFound(format!(
            "The recipe with id: {id} doesn't exist"
        ))),
    }
}

/// Handles the request to retrieve a paginated list of recipes.
///
/// - **Query Parameters**:  
///   - `page` (Optional, u64): The page index, default is 0.
///   - `per_page` (Optional, u64): The number of recipes per page, default is 20.
///
/// - **Response Codes**:  
///   - `200 OK`: Successfully retrieved a list of recipes.
///   - `400 Bad Request`: The request is improperly formatted.
///   - `500 Internal Server Error`: An internal error, most likely related to the database, occurred.
///
/// - **Permissions**:  
///   Non-admin users will only see recipes that are not hidden.
#[utoipa::path(
    get,
    path = "",
    tag = RECIPE_TAG,
    params(
        Pagination,
        RecipeFilterQuery,
        RecipeSortQuery
    ),
    responses(
       (status = 500, description = "An internal error, most likely related to the database, occurred."), 
       (status = 400, description = "The request is improperly formatted."), 
       (status = 200, description = "Successfully retrieved a list of recipes.", body = RecipeListResponse)
    )
)]
pub async fn get_all_recipes(
    _admin: Admin,
    Query(pagination): Query<Pagination>,
    Query(filter): Query<RecipeFilterQuery>,
    Query(sort): Query<RecipeSortQuery>,
    State(conn): State<Connection>,
) -> Result<Json<RecipeListResponse>, AppError> {
    let page = pagination.page.unwrap_or(0);
    let per_page = pagination.per_page.unwrap_or(20);

    let result =
        service::Query::list_recipes_with_condition(&conn, filter.clone(), sort, page, per_page)
            .await?;

    let total_recipes = service::Query::count_recipes_with_condition(&conn, filter).await?;
    let total_page = ((total_recipes.max(1) - 1) / per_page) + 1;

    let recipes = result
        .into_iter()
        .map(|x| x.try_into())
        .collect::<Result<_, RecipeResponseError>>()?;
    Ok(Json(RecipeListResponse {
        current_page: page,
        total_page,
        recipes,
    }))
}
