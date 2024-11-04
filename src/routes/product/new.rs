//! This module contains the route handler for creating a new product.
//!
//! The handler will be accessible via a POST request to the `/product` endpoint.
//! It allows for the creation of new product entries in the database.
//! Admin privileges are required to access this route.

use crate::models::file::FileType;
use crate::models::profile::admin::Admin;
use crate::{error::AppError, routes::utils::openapi::PRODUCT_TAG};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::{models::product::ActiveModel, request::product::NewProductRequest};
use service::Connection;

/// Handler for creating a new product.
///
/// This function allows an admin to create a new product by sending a POST request to the `/product` endpoint.
/// The new product is validated and stored in the database. The image associated with the product is checked in S3 storage.
///
/// - **Admin privileges** are required to access this route.
/// - Returns a `201 Created` status upon successful creation along with the product's ID.
///
/// Path: `/product`
///
/// - **Request Body:** Expects a `NewProduct` JSON object.
/// - **Responses:**
///     - 500: Internal server error (likely database related).
///     - 400: Bad request (invalid input data).
///     - 201: Successfully created a new product, returns the new product's ID as a string.
#[utoipa::path(post, path = "", 
                tag = PRODUCT_TAG,
                request_body(content = NewProductRequest, content_type = "application/json"), 
                responses(
                   (status = 500, description = "An internal error, most likely related to the database, occurred."), 
                   (status = 400, description = "The request is improperly formatted."), 
                   (status = 201, description = "Successfully created a new product, returns the new product's ID as a string.", body = uuid::Uuid)
                )
               )]
pub async fn post_new_product(
    admin: Admin,
    State(conn): State<Connection>,
    State(s3): State<s3::Bucket>,
    Json(product): Json<NewProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if image exist
    if let Some(image) = product.image.clone() {
        let (_result, _code) = s3
            .head_object(format!("{}/{}", FileType::Product, image))
            .await?;
    }

    let product_model: ActiveModel = product.try_into()?;
    let result = service::Mutation::create_product(&conn, product_model).await?;

    let id = result.id;

    tracing::info!(
        "{admin} added a new product {} \"{}\" - {:?}",
        result.name,
        id,
        result
    );

    Ok((StatusCode::CREATED, id.to_string()).into_response())
}
