//! Utility functions for handling file downloads.
//!
//! This module provides functions to facilitate the downloading of files
//! from the server, including setting appropriate headers and streaming
//! file content to the client. It aims to simplify the process of
//! serving files in a web application.

use super::openapi::MISC_TAG;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, header::CACHE_CONTROL},
    response::IntoResponse,
};
use entity::error::AppError;
use service::s3::FileParams;

/// Downloads files from the server's storage.
///
/// This function retrieves a specified file based on the given filename
/// and type, and streams it to the client. It also handles errors
/// related to file access, returning appropriate HTTP status codes
/// for various scenarios such as file not found or other errors.
///
/// # Parameters
/// - `filename`: The name of the file to be downloaded.
/// - `params`: The type of the file, which affects the download behavior.
///
/// # Responses
/// - `200`: The file is correctly uploaded.
/// - `400`: You're missing some field.
///
/// # Errors
/// Returns an error if the file does not exist or if there is an issue
/// with the S3 storage.
#[utoipa::path(
    get,
    path = "/download/{filename}", 
    tag = MISC_TAG,
    params(
        ("filename" = String, Path, description = "The filename"),
        FileParams,
    ),
    responses(
        (status = 200, description = "Successfully retrieved the file", body = String),
        (status = 400, description = "You're missing some field"),
        (status = 500, description = "An internal error, most likely related to s3, occurred."), 
    ),
)]
pub async fn download_file(
    Path(filename): Path<String>,
    State(conn): State<entity::s3::S3FileStorage>,
    params: Query<FileParams>,
) -> Result<impl IntoResponse, AppError> {
    let path = format!("{}/{filename}", params.file_type);

    let stream = conn
        .client
        .get_object()
        .bucket(&conn.bucket)
        .key(path)
        .send()
        .await
        .map_err(|err| match err.into_service_error() {
            aws_sdk_s3::operation::get_object::GetObjectError::NoSuchKey(_not_found) => {
                AppError::NotFound(format!("The file you've asked doesn't exist: {filename}"))
            }
            err => err.into(),
        })?;
    let content = stream.body.collect().await?.into_bytes();
    let body = axum::body::Body::from(content);

    let headers = HeaderMap::from_iter([(
        CACHE_CONTROL,
        HeaderValue::from_static("max-age=2629746"), // 1 Month
    )]);
    Ok((headers, body))
}
