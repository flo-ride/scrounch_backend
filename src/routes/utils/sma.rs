//! This module provides the functionality to interact with the Sma API for importing
//! existing products into the system. It handles communication with the Sma API,
//! processing the data, and updating the local database with imported products.

use super::openapi::MISC_TAG;
use crate::Arguments;
use axum::{
    extract::{Query, State},
    Json,
};
use entity::{
    error::AppError,
    models::product::{self, Model as Product},
    response::{
        product::{EditedProductResponse, ProductResponse, ProductResponseError},
        sma::SmaResponse,
    },
};
use extractor::profile::admin::Admin;
use futures::future::join_all;
use sea_orm::ActiveValue::Set;
use service::{s3::FileType, Connection};

/// Enum representing changes in SMA products.
///
/// `SmaChange` tracks the state of products, indicating whether a product
/// was unchanged, edited, or newly created.
///
/// - `Unchanged`: The product remains the same.
/// - `Edited`: The product has been edited, with updated information in `EditedProductResponse`.
/// - `Created`: The product is new and has been added to the system.
#[derive(Debug, Clone, PartialEq)]
pub enum SmaChange {
    Unchanged(Product),
    Edited(EditedProductResponse),
    Created(Product),
}

/// Struct defining the fields that determine changes in a product.
///
/// `SmaChangeTypeMatrix` specifies which attributes of a product (e.g., `name` or `price`)
/// have changed. Each field is a boolean representing whether the corresponding attribute
/// has changed (`true`) or remained the same (`false`).
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct SmaChangeTypeMatrix {
    /// Indicates if the product's name has changed.
    #[serde(default)]
    pub name: bool,

    /// Indicates if the product's price has changed.
    #[serde(default)]
    pub price: bool,
}

/// Struct representing a collection of SMA products, used for pagination purposes.
///
/// `SmaProducts` holds a list of products with metadata, including total count, current limit,
/// and start position for pagination.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SmaProducts {
    /// List of SMA products.
    pub data: Vec<SmaProduct>,

    /// Maximum number of products returned per page.
    pub limit: serde_json::Value,

    /// Start index for the current set of products.
    pub start: serde_json::Value,

    /// Total number of available products.
    pub total: u64,
}

/// Struct representing an individual SMA product with detailed attributes.
///
/// `SmaProduct` includes product details such as category, price, tax rate, and unit.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SmaProduct {
    /// Category details of the product.
    pub category: SmCategory,

    /// Unique code identifying the product.
    pub code: String,

    /// Unique identifier of the product.
    pub id: String,

    /// Optional URL of the product's image.
    pub image_url: Option<String>,

    /// Name of the product.
    pub name: String,

    /// Net price of the product.
    pub net_price: String,

    /// Final price of the product including applicable taxes.
    pub price: String,

    /// Slug for URL-friendly product identification.
    pub slug: String,

    /// Tax method used for the product.
    pub tax_method: String,

    /// Tax rate details for the product.
    pub tax_rate: SmaTaxRate,

    /// Type of the product.
    #[serde(rename = "type")]
    pub type_field: String,

    /// Unit of measurement for the product.
    pub unit: SmaUnit,

    /// Price per unit of the product.
    pub unit_price: String,
}

/// Struct representing the category of an SMA product.
///
/// `SmCategory` provides metadata about the product's category, including name,
/// description, and parent category ID.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SmCategory {
    /// Unique identifier of the category.
    pub id: String,

    /// Unique code identifying the category.
    pub code: String,

    /// Name of the category.
    pub name: String,

    /// Optional URL of the category's image.
    pub image: Option<String>,

    /// Identifier of the parent category, if any.
    pub parent_id: String,

    /// Slug for URL-friendly category identification.
    pub slug: String,

    /// Description of the category.
    pub description: String,
}

/// Struct representing the tax rate associated with an SMA product.
///
/// `SmaTaxRate` includes tax attributes such as rate, type, and unique identifiers.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SmaTaxRate {
    /// Unique identifier of the tax rate.
    pub id: String,

    /// Name of the tax rate.
    pub name: String,

    /// Unique code identifying the tax rate.
    pub code: String,

    /// Rate of the tax as a percentage or absolute value.
    pub rate: String,

    /// Type of the tax rate.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// Struct representing the unit of measurement for an SMA product.
///
/// `SmaUnit` provides details such as unit name, base unit, and any applicable
/// operations or values for unit conversion.
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SmaUnit {
    /// Unique identifier of the unit.
    pub id: String,

    /// Code identifying the unit.
    pub code: String,

    /// Name of the unit.
    pub name: String,

    /// Optional base unit for conversion.
    pub base_unit: Option<String>,

    /// Optional operator for unit conversion.
    pub operator: Option<String>,

    /// Optional value for unit conversion.
    pub unit_value: Option<String>,

    /// Optional operation value for unit conversion.
    pub operation_value: Option<String>,
}

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
#[utoipa::path(
    post,
    path = "/sma", 
    tag = MISC_TAG,
    responses(
        (status = 500, description = "An internal error, most likely related to the database, occurred."), 
        (status = 400, description = "The request is improperly formatted."), 
        (status = 201, description = "Successfully updated every Sma Products", body = SmaResponse)
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn post_update_from_sma(
    admin: Admin,
    State(arguments): State<Arguments>,
    State(conn): State<Connection>,
    State(s3): State<s3::Bucket>,
    Query(params): Query<SmaChangeTypeMatrix>,
) -> Result<Json<SmaResponse>, AppError> {
    tracing::info!("{admin} just asked for an SMA update",);

    match (arguments.sma_api_key, arguments.sma_url) {
        (Some(api_key), Some(url)) => {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "api-key",
                reqwest::header::HeaderValue::from_str(&api_key).map_err(|err| {
                    AppError::InternalError(format!("Cannot map api_key to HeaderValue - {err}"))
                })?,
            );

            let client = reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .map_err(|err| {
                    AppError::InternalError(format!("Cannot build HTTP Client - {err}"))
                })?;

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
                        AppError::InternalError(format!(
                            "Sorry it seems that we cannot contact the SMA Api - {err}"
                        ))
                    })?
                    .json::<SmaProducts>()
                    .await
                    .map_err(|err| {
                        AppError::InternalError(format!(
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
            Err(AppError::InternalError("Sorry but it seems all SMA variables are not filled, Please contact your Website Admin".to_string()))
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
                        AppError::InternalError(format!(
                            "Cannot convert price: {} - {err}",
                            product.price
                        ))
                    })?;

                if price != existing_product.price {
                    let u64_price = price.try_into().map_err(|err| {
                        AppError::InternalError(format!(
                            "Cannot convert price into u64: {price} - {err}"
                        ))
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
                let sma_filename = image_url.split("/").last().ok_or(AppError::InternalError(
                    "Cannot find SMA filename".to_string(),
                ))?;

                let extension = std::path::Path::new(&sma_filename)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .ok_or(AppError::InternalError(format!(
                        "SMA file is missing an extension: {sma_filename}"
                    )))?;

                let image = reqwest::get(image_url.clone()).await.map_err(|err| {
                    AppError::InternalError(format!("Cannot download image from SMA: {err}"))
                })?;

                let name = format!("{}.{extension}", uuid::Uuid::new_v4());
                let s3_path = format!("{}/{name}", FileType::Product);
                let image = image.bytes().await.map_err(|err| {
                    AppError::InternalError(format!("Cannot get bytes of image - {err}"))
                })?;
                s3.put_object(s3_path, &image).await?;

                filename = Some(name.clone());

                tracing::info!("Adding new product Image from SMA: {name}");
            }

            let form_data = product::ActiveModel {
                id: Set(uuid::Uuid::new_v4()),

                name: Set(product.name),

                image: Set(filename),

                price: Set(
                    sea_orm::prelude::Decimal::from_str_exact(&product.price).map_err(|err| {
                        AppError::InternalError(format!(
                            "Cannot convert price: {} - {err}",
                            product.price
                        ))
                    })?,
                ),

                price_currency: Set(entity::models::sea_orm_active_enums::Currency::Euro),

                sma_code: Set(Some(product.code)),
                ..Default::default()
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
