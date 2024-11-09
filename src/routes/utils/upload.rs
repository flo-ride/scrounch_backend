//! File upload utilities for the `scrounch_backend` application.
//!
//! This module provides functionality for handling file uploads, typically used
//! for processing user-uploaded content such as images, documents, or other assets.

use axum::{
    extract::{Multipart, Query, State},
    Json,
};
use entity::error::AppError;
use extractor::profile::admin::Admin;
use futures::stream::TryStreamExt;
use service::s3::FileParams;

use super::openapi::MISC_TAG;
#[derive(utoipa::ToSchema)]
pub struct FileSchema {
    #[allow(dead_code)]
    #[schema(content_media_type = "application/octet-stream")]
    file_bytes: Vec<u8>,
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
    request_body(content = inline(FileSchema), content_type = "multipart/form-data"),
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
    State(conn): State<s3::Bucket>,
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

        let mimetype = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let body_with_io_error =
            field.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));

        let reader = tokio_util::io::StreamReader::new(body_with_io_error);

        futures::pin_mut!(reader);

        let new_filename = match extension {
            Some(extension) => format!("{}.{extension}", uuid::Uuid::new_v4()),
            None => uuid::Uuid::new_v4().to_string(),
        };
        let s3_path = format!("{}/{new_filename}", params.file_type);
        conn.put_object_stream_with_content_type(&mut reader, &s3_path, &mimetype)
            .await?;
        conn.put_object_tagging(
            &s3_path,
            &[
                ("Author", &user.id.to_string()),
                ("Type", &params.file_type.to_string()),
            ],
        )
        .await?;

        tracing::info!("{user} just uploaded a new file: \"{filename}\" -> \"{s3_path}\"",);
        result.push((filename, new_filename));
    }
    Ok(Json(result))
}
