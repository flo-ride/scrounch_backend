//! Route for editing an existing product in the store.

use crate::{
    error::AppError,
    models::{file::FileType, profile::admin::Admin, request::product::EditProduct},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::product::Model as Product;
use service::Connection;

/// Edit an existing product by ID in the store.
///
/// The admin can change attributes such as the name, price, quantity, or image of the product.
/// If the product image is changed, the old image will be deleted from S3 storage.
///
/// Returns an error if the product doesn't exist, if there is a validation issue, or if a database or S3 operation fails.
#[utoipa::path(put, path = "/product/{id}",
               params(
                   ("id" = uuid::Uuid, Path, description = "Product database id to edit product for"),
                ),
               request_body(content = EditProduct, content_type = "application/json"), 
               responses(
                   (status = 500, description = "An internal error occured, probably database related"), 
                   (status = 400, description = "Your request is not correctly formatted"), 
                   (status = 200, description = "The product is correctly edited")
                )
               )]
pub async fn edit_product(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
    State(s3): State<s3::Bucket>,
    Json(new_product): Json<EditProduct>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_product_by_id(&conn, id).await?;

    match result {
        Some(existing_product) => {
            let old_image = existing_product.image.clone();
            match (new_product.image.clone(), old_image.clone()) {
                (Some(new_image), Some(old_image)) => {
                    if new_image != old_image {
                        // Check if new image exist
                        let (_result, _code) = s3
                            .head_object(format!("{}/{}", FileType::Product, new_image))
                            .await?;
                    }
                }
                (Some(new_image), _) => {
                    // Check if new image exist
                    let (_result, _code) = s3
                        .head_object(format!("{}/{}", FileType::Product, new_image))
                        .await?;
                }
                _ => {}
            }

            service::Mutation::update_product(
                &conn,
                id,
                Product {
                    id,
                    image: match new_product.image.clone() {
                        None => existing_product.image.clone(),
                        Some(image) => Some(image),
                    },
                    name: match new_product.name.clone() {
                        None => existing_product.name.clone(),
                        Some(name) => {
                            let max_length = 32;
                            if name.is_empty() {
                                return Err(AppError::BadOption(
                                    "Name cannot be empty".to_string(),
                                ));
                            }
                            if name.len() > max_length {
                                return Err(AppError::BadOption(format!(
                                    "Name cannot be longer than {max_length}: {name}",
                                )));
                            }
                            name
                        }
                    },
                    price: match new_product.price {
                        None => existing_product.price,
                        Some(price) => {
                            if price <= 0.0 {
                                return Err(AppError::BadOption(format!(
                                    "You cannot put a null / negative price: {price}",
                                )));
                            }
                            sea_orm::prelude::Decimal::from_str_exact(&price.to_string()).map_err(
                                |err| {
                                    AppError::Unknow(format!(
                                        "Cannot convert price: {price} - {err}",
                                    ))
                                },
                            )?
                        }
                    },
                    quantity: match new_product.quantity {
                        None => existing_product.quantity,
                        Some(quantity) => {
                            if quantity > 9999 {
                                return Err(AppError::BadOption(format!(
                                    "Quantity is too big: {quantity}"
                                )));
                            }

                            quantity.try_into().map_err(|err| {
                                AppError::Unknow(format!(
                                    "Quantity cannot be converted: {quantity} - {err}",
                                ))
                            })?
                        }
                    },
                    max_quantity_per_command: match new_product.max_quantity_per_command {
                        None => existing_product.max_quantity_per_command,
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
                    disabled: new_product.disabled.unwrap_or(false),
                    creation_time: chrono::offset::Local::now().into(),
                },
            )
            .await?;

            if let (Some(new_image), Some(old_image)) =
                (new_product.image.clone(), old_image.clone())
            {
                if new_image != old_image {
                    s3.delete_object(format!("{}/{}", FileType::Product, old_image))
                        .await?;
                }
            }
            tracing::info!(
                "Admin {} \"{}\" successfully edited product {} \"{}\" - {:?}",
                admin.name,
                admin.id,
                existing_product.name,
                id,
                new_product
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The product with id: {id} doesn't exist"
        ))),
    }
}
