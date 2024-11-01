//! This module provides the functionality to interact with the Sma API for importing
//! existing products into the system. It handles communication with the Sma API,
//! processing the data, and updating the local database with imported products.

use axum::extract::{Query, State};
use axum::Json;
use entity::response::product::{EditedProductResponse, ProductResponse, ProductResponseError};
use entity::response::sma::SmaResponse;
use futures::future::join_all;
use service::Connection;

use crate::models::utils::sma::SmaChangeTypeMatrix;
use crate::{
    error::AppError,
    models::{
        file::FileType,
        profile::admin::Admin,
        utils::sma::{SmaChange, SmaProduct, SmaProducts},
    },
    Arguments,
};

/// Updates the local product database by importing products from the Sma API.
/// This function retrieves the latest products from Sma, processes the data,
/// and updates the local products accordingly.
///
/// # Errors
/// - Returns a 500 status code if there is an internal error, such as a failure to fetch or process Sma data.
/// - Returns a 400 status code if the request to Sma is not correctly formatted.
///
/// # Responses
/// - 200: The products have been successfully updated.
pub async fn post_update_from_sma(
    admin: Admin,
    State(arguments): State<Arguments>,
    State(conn): State<Connection>,
    State(s3): State<s3::Bucket>,
    Query(params): Query<SmaChangeTypeMatrix>,
) -> Result<Json<SmaResponse>, AppError> {
    tracing::info!(
        "{} \"{}\" just asked for an SMA update",
        admin.name,
        admin.id
    );

    match (arguments.sma_api_key, arguments.sma_url) {
        (Some(api_key), Some(url)) => {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "api-key",
                reqwest::header::HeaderValue::from_str(&api_key).map_err(|err| {
                    AppError::Unknow(format!("Cannot map api_key to HeaderValue - {err}"))
                })?,
            );

            let client = reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .map_err(|err| AppError::Unknow(format!("Cannot build HTTP Client - {err}")))?;

            let mut products: Vec<SmaChange> = Vec::new();

            let mut start = 0;
            let limit = 20;

            loop {
                let sma_result = client
                    .get(format!(
                        "{url}/api/v1/products?start={start}&limit={limit}&include=category"
                    ))
                    .send()
                    .await
                    .map_err(|err| {
                        AppError::Unknow(format!(
                            "Sorry it seems that we cannot contact the SMA Api - {err}"
                        ))
                    })?
                    .json::<SmaProducts>()
                    .await
                    .map_err(|err| {
                        AppError::Unknow(format!(
                            "Cannot Deserialize SMA Response into json - {err}"
                        ))
                    })?;

                let total = sma_result.total;

                let result = join_all(
                    sma_result
                        .data
                        .into_iter()
                        .filter(|x| {
                            if let Some(categories) = arguments.sma_categories.clone() {
                                categories.contains(&x.category.code)
                            } else {
                                true
                            }
                        })
                        .map(|x| async {
                            create_or_update_sma_product(&conn, &s3, x, params).await
                        }),
                )
                .await
                .into_iter()
                .collect::<Result<Vec<_>, AppError>>()?;

                products.extend(result);

                start += limit;
                if start + limit > total {
                    break;
                }
                // This is just the rust idiomatic way of a do_while
            }

            let iter = products.into_iter();
            Ok(Json(SmaResponse {
                unchanged: iter
                    .clone()
                    .filter_map(|x| match x {
                        SmaChange::Unchanged(x) => Some(x),
                        _ => None,
                    })
                    .map(|x| x.id)
                    .collect(),
                changed: iter
                    .clone()
                    .filter_map(|x| match x {
                        SmaChange::Edited(x) => Some(x),
                        _ => None,
                    })
                    .collect(),
                created: iter
                    .filter_map(|x| match x {
                        SmaChange::Created(x) => Some(x),
                        _ => None,
                    })
                    .map(TryInto::<ProductResponse>::try_into)
                    .collect::<Result<_, ProductResponseError>>()?,
            }))
        }
        _ => {
            tracing::error!("Sorry but it seems all SMA variables are not filled");
            Err(AppError::MissingOption("Sorry but it seems all SMA variables are not filled, Please contact your Website Admin".to_string()))
        }
    }
}

async fn create_or_update_sma_product(
    conn: &Connection,
    s3: &s3::Bucket,
    product: SmaProduct,
    overwrite_matrix: SmaChangeTypeMatrix,
) -> Result<SmaChange, AppError> {
    let result = service::Query::find_product_by_sma_code(conn, product.code.clone()).await?;

    match result {
        Some(existing_product) => {
            let mut is_change = false;
            let mut changes = EditedProductResponse::default();
            let mut edited_product = existing_product.clone();

            if overwrite_matrix.name && product.name != existing_product.name {
                is_change = true;
                changes.name = Some(product.name.clone());
                edited_product.name = product.name;
            }

            if overwrite_matrix.price {
                let price =
                    sea_orm::prelude::Decimal::from_str_exact(&product.price).map_err(|err| {
                        AppError::Unknow(format!("Cannot convert price: {} - {err}", product.price))
                    })?;

                if price != existing_product.price {
                    let u64_price = price.try_into().map_err(|err| {
                        AppError::Unknow(format!("Cannot convert price into u64: {price} - {err}"))
                    })?;
                    is_change = true;
                    changes.price = Some(u64_price);
                    edited_product.price = price;
                }
            }

            match is_change {
                true => {
                    service::Mutation::update_product(conn, existing_product.id, edited_product)
                        .await?;
                    Ok(SmaChange::Edited(changes))
                }
                false => Ok(SmaChange::Unchanged(existing_product)),
            }
        }
        None => {
            let mut filename: Option<String> = None;

            if let Some(image_url) = product.image_url {
                let sma_filename = image_url
                    .split("/")
                    .last()
                    .ok_or(AppError::Unknow("Cannot find SMA filename".to_string()))?;

                let extension = std::path::Path::new(&sma_filename)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .ok_or(AppError::Unknow(format!(
                        "SMA file is missing an extension: {sma_filename}"
                    )))?;

                let image = reqwest::get(image_url.clone()).await.map_err(|err| {
                    AppError::Unknow(format!("Cannot download image from SMA: {err}"))
                })?;

                let name = format!("{}.{extension}", uuid::Uuid::new_v4());
                let s3_path = format!("{}/{name}", FileType::Product);
                let image = image.bytes().await.map_err(|err| {
                    AppError::Unknow(format!("Cannot get bytes of image - {err}"))
                })?;
                s3.put_object(s3_path, &image).await?;

                filename = Some(name.clone());

                tracing::info!("Adding new product Image from SMA: {name}");
            }

            let form_data = entity::models::product::Model {
                id: uuid::Uuid::new_v4(),

                name: product.name,

                image: filename,

                price: sea_orm::prelude::Decimal::from_str_exact(&product.price).map_err(
                    |err| {
                        AppError::Unknow(format!("Cannot convert price: {} - {err}", product.price))
                    },
                )?,

                sma_code: Some(product.code),

                creation_time: chrono::offset::Local::now().into(),

                max_quantity_per_command: None,

                disabled: false,
            };
            let result = service::Mutation::create_product(conn, form_data).await?;
            tracing::info!(
                "Adding new product from SMA: {} \"{}\" - {result:?}",
                result.name,
                result.id
            );

            Ok(SmaChange::Created(result))
        }
    }
}
