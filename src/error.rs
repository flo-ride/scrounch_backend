//! This module defines the `AppError` struct, which define common Error for the `scrounch_backend` application.

/// Custom error type for the `scrounch_backend` application.
///
/// This enum defines the error types used within the application, allowing for easy handling
/// of different error scenarios. Currently, it supports:
/// - `DatabaseError`: Represents an error occurring while interacting with the database.
///
/// The `AppError` type is used throughout the application to simplify error management and
/// provide more meaningful error messages when something goes wrong.
#[derive(Debug)]
pub enum AppError {
    S3Error(s3::error::S3Error),
    MultipartError(axum::extract::multipart::MultipartError),
    MissingOption(String),
    BadOption(String),
    DatabaseError(sea_orm::DbErr),
    OidcError(axum_oidc::error::ExtractorError),
    Unknow(String),
    Forbidden,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::DatabaseError(value) => write!(f, "DatabaseError: {value}"),
            Self::OidcError(value) => write!(f, "OidcError: {value}"),
            Self::S3Error(value) => write!(f, "S3Error: {value}"),
            Self::MultipartError(value) => write!(f, "MultipartError: {value}"),
            Self::MissingOption(value) => write!(f, "MissingOption: {value}"),
            Self::BadOption(value) => write!(f, "BadOption: {value}"),
            Self::Unknow(value) => write!(f, "Unknow Error - This should NEVER happened - {value}"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::DatabaseError(value)
    }
}

impl From<axum_oidc::error::ExtractorError> for AppError {
    fn from(value: axum_oidc::error::ExtractorError) -> Self {
        Self::OidcError(value)
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(value: axum::extract::multipart::MultipartError) -> Self {
        Self::MultipartError(value)
    }
}

impl From<s3::error::S3Error> for AppError {
    fn from(value: s3::error::S3Error) -> Self {
        Self::S3Error(value)
    }
}
