//! File upload utilities for the `scrounch_backend` application.
//!
//! This module provides functionality for handling file uploads, typically used
//! for processing user-uploaded content such as images, documents, or other assets.

use axum::{
    Json,
    extract::{Multipart, Query, State},
};
use entity::error::AppError;
use extractor::profile::admin::Admin;
use service::s3::FileParams;

use super::openapi::MISC_TAG;

/// Represents a file schema used in API interactions, typically for file uploads.
///
/// The `FileSchema` struct is used to define a file's content in an API endpoint,
/// particularly for handling binary file uploads. It includes the file's content
/// as a byte vector, which can be processed by the server during file transfer.
#[derive(utoipa::ToSchema)]
pub struct FileSchema {
    /// Represents the file data
    #[allow(dead_code)]
    #[schema(value_type = String, format = Binary)]
    file: std::fs::File,
}

/// Upload files
///
/// This endpoint allows an admin user to upload files to a specified S3 bucket.
/// It uses multipart form data to handle file uploads and stores them in a temporary S3 directory.
#[utoipa::path(
    post,
    path = "/upload", 
    tag = MISC_TAG,
    params(
        FileParams
    ),
    request_body(content = FileSchema, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "The file is correctly uploaded", body = String),
        (status = 400, description = "You're missing some field"),
        (status = 500, description = "An internal error, most likely related to s3, occurred."), 
    ),
    security(
        ("axum-oidc" = [])
    )
)]
pub async fn post_upload_files(
    user: Admin,
    State(conn): State<entity::s3::S3FileStorage>,
    params: Query<FileParams>,
    mut multipart: Multipart,
) -> Result<Json<Vec<(String, String)>>, AppError> {
    let mut result: Vec<(String, String)> = vec![];
    while let Ok(Some(field)) = multipart.next_field().await {
        let filename = field.file_name().unwrap_or("").to_string();
        let extension = std::path::Path::new(&filename)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .or(None);

        let bytes = field.bytes().await?;
        let byte_stream = aws_sdk_s3::primitives::ByteStream::from(bytes);

        let new_filename = match extension {
            Some(extension) => format!("{}.{extension}", uuid::Uuid::new_v4()),
            None => uuid::Uuid::new_v4().to_string(),
        };
        let s3_path = format!("{}/{new_filename}", params.file_type);
        conn.client
            .put_object()
            .bucket(&conn.bucket)
            .key(&s3_path)
            .body(byte_stream)
            .send()
            .await?;

        log::info!("{user} just uploaded a new file: \"{filename}\" -> \"{s3_path}\"",);
        result.push((filename, new_filename));
    }
    Ok(Json(result))
}
