//! File upload utilities for the `scrounch_backend` application.
//!
//! This module provides functionality for handling file uploads, typically used
//! for processing user-uploaded content such as images, documents, or other assets.

use axum::{
    extract::{Multipart, Query, State},
    Json,
};
use futures::stream::TryStreamExt;

use crate::{
    error::AppError,
    models::{admin::Admin, file::FileType},
};
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize)]
pub struct UploadParams {
    pub upload_type: FileType,
}

#[derive(utoipa::ToSchema)]
pub struct FileSchema {
    #[schema(format = Binary, value_type = String)]
    #[allow(dead_code)]
    image_bytes: Vec<u8>,
}

/// Upload files
///
/// This endpoint allows an admin user to upload files to a specified S3 bucket.
/// It uses multipart form data to handle file uploads and stores them in a temporary S3 directory.
#[utoipa::path(post, path = "/upload", 
        params(
            ("upload_type" = FileType, Path, description = "The type of uploaded file")
        ),
        responses(
            (status = 200, description = "The file is correctly uploaded"),
            (status = 400, description = "You're missing some field")
        ),
        request_body(content = FileSchema, content_type = "multipart/form-data")

        )]
pub async fn post_upload_files(
    user: Admin,
    State(conn): State<s3::Bucket>,
    params: Query<UploadParams>,
    mut multipart: Multipart,
) -> Result<Json<Vec<(String, String)>>, AppError> {
    let mut result: Vec<(String, String)> = vec![];
    while let Some(field) = multipart.next_field().await? {
        let name = field
            .name()
            .ok_or(AppError::MissingOption(
                "Multipart Field is missing name".to_string(),
            ))?
            .to_string();
        let filename = field
            .file_name()
            .ok_or(AppError::MissingOption(
                "Multipart Field is missing filename".to_string(),
            ))?
            .to_string();
        let extension = std::path::Path::new(&filename)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or(AppError::MissingOption(format!(
                "Multipart file is missing an extension: {filename}"
            )))?;
        let mimetype = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let body_with_io_error =
            field.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));

        let reader = tokio_util::io::StreamReader::new(body_with_io_error);

        futures::pin_mut!(reader);

        let new_filename = format!("{}_{name}.{extension}", uuid::Uuid::new_v4());
        let s3_path = format!("{}/{new_filename}", params.upload_type);
        conn.put_object_stream_with_content_type(&mut reader, &s3_path, &mimetype)
            .await?;
        conn.put_object_tagging(
            &s3_path,
            &[
                ("Author", &user.id),
                ("Type", &params.upload_type.to_string()),
            ],
        )
        .await?;

        tracing::info!(
            "\"{}\" ({}) just uploaded a new file: \"{filename}\" -> \"{s3_path}\"",
            user.name,
            user.id
        );
        result.push((filename, new_filename));
    }
    Ok(Json(result))
}