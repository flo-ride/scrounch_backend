//! This module contains the route handler for creating a new product.
//!
//! The handler will be accessible via a POST request to the `/product` endpoint.
//! It allows for the creation of new product entries in the database.
//! Admin privileges are required to access this route.

use crate::models::file::FileType;
use crate::models::profile::admin::Admin;
use crate::{error::AppError, models::request::product::NewProduct};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::product::Model as Product;
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
#[utoipa::path(post, path = "/product", 
               request_body(content = NewProduct, content_type = "application/json"), 
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
    Json(product): Json<NewProduct>,
) -> Result<impl IntoResponse, AppError> {
    // Check if image exist
    if let Some(image) = product.image.clone() {
        let (_result, _code) = s3
            .head_object(format!("{}/{}", FileType::Product, image))
            .await?;
    }

    let id = uuid::Uuid::new_v4();
    service::Mutation::create_product(
        &conn,
        Product {
            id,
            image: product.image.clone(),
            name: {
                let name = product.name.clone();
                let max_length = 32;
                if name.is_empty() {
                    return Err(AppError::BadOption("Name cannot be empty".to_string()));
                }
                if name.len() > max_length {
                    return Err(AppError::BadOption(format!(
                        "Name cannot be longer than {max_length}: {name}",
                    )));
                }
                name
            },
            price: {
                let price = product.price;
                if price <= 0.0 {
                    return Err(AppError::BadOption(format!(
                        "You cannot put a null / negative price: {price}",
                    )));
                }
                sea_orm::prelude::Decimal::from_str_exact(&price.to_string()).map_err(|err| {
                    AppError::Unknow(format!("Cannot convert price: {price} - {err}"))
                })?
            },
            quantity: {
                let quantity = product.quantity;
                if quantity > 9999 {
                    return Err(AppError::BadOption(format!(
                        "Quantity is too big: {quantity}"
                    )));
                }

                quantity.try_into().map_err(|err| {
                    AppError::Unknow(format!("Quantity cannot be converted: {quantity} - {err}",))
                })?
            },
            max_quantity_per_command: match product.max_quantity_per_command {
                None => None,
                Some(0) => None,
                Some(x) => {
                    if x > 10 {
                        return Err(AppError::BadOption(format!(
                            "Max Quantity Per Commmand is too big: {x}"
                        )));
                    }

                    Some(x.try_into().map_err(|err| {
                        AppError::Unknow(format!(
                            "Max Quantity Per Commmand is cannot be converted: {x} - {err}"
                        ))
                    })?)
                }
            },
            disabled: false,
            creation_time: chrono::offset::Local::now().into(),
        },
    )
    .await?;

    tracing::info!(
        "Admin {} \"{}\" added a new product {} \"{}\" - {:?}",
        admin.name,
        admin.id,
        product.name,
        id,
        product
    );

    Ok((StatusCode::CREATED, id.to_string()).into_response())
}
