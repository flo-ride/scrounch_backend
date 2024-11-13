//! Route for editing an existing product in the store.

use crate::utils::openapi::PRODUCT_TAG;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{error::AppError, models::product::ActiveModel, request::product::EditProductRequest};
use extractor::profile::admin::Admin;
use service::{s3::FileType, Connection};

/// Edit an existing product by ID in the store.
///
/// The admin can change attributes such as the name, price, quantity, or image of the product.
/// If the product image is changed, the old image will be deleted from S3 storage.
///
/// Returns an error if the product doesn't exist, if there is a validation issue, or if a database or S3 operation fails.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = PRODUCT_TAG,
    params(
        ("id" = uuid::Uuid, Path, description = "Product database id to edit product for"),
    ),
    request_body(content = EditProductRequest, content_type = "application/json"), 
    responses(
        (status = 500, description = "An internal error occured, probably database related"), 
        (status = 400, description = "Your request is not correctly formatted"), 
        (status = 200, description = "The product is correctly edited")
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn edit_product(
    admin: Admin,
    Path(id): Path<uuid::Uuid>,
    State(conn): State<Connection>,
    State(s3): State<s3::Bucket>,
    Json(edit_product): Json<EditProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = service::Query::find_product_by_id(&conn, id).await?;

    match result {
        Some(existing_product) => {
            let edit_product: ActiveModel = edit_product.try_into()?;

            let mut check_image = None;
            let mut delete_image = None;
            if let sea_orm::ActiveValue::Set(image_change) = edit_product.image.clone() {
                match image_change {
                    Some(new_image) => match existing_product.image {
                        Some(existing_image) => {
                            if new_image != existing_image {
                                check_image = Some(new_image);
                                delete_image = Some(existing_image);
                            }
                        }
                        None => check_image = Some(new_image),
                    },
                    None => {
                        if let Some(image) = existing_product.image {
                            delete_image = Some(image);
                        }
                    }
                }
            }

            if let Some(image) = check_image {
                let (_result, _code) = s3
                    .head_object(format!("{}/{}", FileType::Product, image))
                    .await
                    .map_err(|_| {
                        entity::request::product::ProductRequestError::ImageDoesNotExist(image)
                    })?;
            }

            let result = service::Mutation::update_product(&conn, id, edit_product).await?;

            if let Some(image) = delete_image {
                s3.delete_object(format!("{}/{}", FileType::Product, image))
                    .await?;
            }

            log::info!(
                "{admin} successfully edited product {} \"{}\" - {:?}",
                existing_product.name,
                id,
                result
            );

            Ok((StatusCode::OK, ""))
        }
        None => Err(AppError::NotFound(format!(
            "The product with id: {id} doesn't exist"
        ))),
    }
}
