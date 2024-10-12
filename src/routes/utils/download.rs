//! Utility functions for handling file downloads.
//!
//! This module provides functions to facilitate the downloading of files
//! from the server, including setting appropriate headers and streaming
//! file content to the client. It aims to simplify the process of
//! serving files in a web application.

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header::CACHE_CONTROL, HeaderMap, HeaderValue},
    response::IntoResponse,
};
use s3::error::S3Error;

use crate::{error::AppError, models::file::FileParams};

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
#[utoipa::path(get, path = "/download/{filename}", 
        params(
            ("filename" = String, Path, description = "The filename"),
            ("type" = FileType, Query, description = "The type of downloaded file")
        ),
        responses(
            (status = 200, description = "Successfully retrieved the file", body = String),
            (status = 400, description = "You're missing some field"),
            (status = 500, description = "An internal error, most likely related to s3, occurred."), 
        ),
    )]
pub async fn download_file(
    Path(filename): Path<String>,
    State(conn): State<s3::Bucket>,
    params: Query<FileParams>,
) -> Result<impl IntoResponse, AppError> {
    let path = format!("{}/{filename}", params.file_type);

    let stream = conn
        .get_object_stream(path)
        .await
        .map_err(|err| match err {
            S3Error::HttpFailWithBody(404, _body) => {
                AppError::NotFound(format!("The file you've asked doesn't exist: {filename}"))
            }
            _ => AppError::S3Error(err),
        })?;
    let body = Body::from_stream(stream.bytes);

    let headers = HeaderMap::from_iter([(
        CACHE_CONTROL,
        HeaderValue::from_static("max-age=2629746"), // 1 Month
    )]);
    Ok((headers, body))
}
