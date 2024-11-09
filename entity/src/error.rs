//! Module for error handling in the `scrounch_backend` entity layer.
//!
//! This module consolidates and manages errors specific to the entity layer,
//! providing a standardized way to represent and handle errors that occur
//! //!  It includes various custom error types to aid in debugging and error
//! tracking throughout the application.

use axum::http::StatusCode;

/// Macro to implement `From<$source_type>` for `AppError` when the error needs to be converted into a string.
/// This is useful for errors that can be represented as a string message.
///
/// # Example:
/// ```rust
/// impl_from_error_to_string!(SomeErrorType, InternalError);
/// ```
macro_rules! impl_from_error_to_string {
    ($source_type:ty, $variant:ident) => {
        impl From<$source_type> for crate::error::AppError {
            fn from(value: $source_type) -> Self {
                Self::$variant(value.to_string())
            }
        }
    };
}

/// Macro to implement `From<$source_type>` for `AppError` when the error is directly converted into an `AppError` variant.
/// This is useful when the error type can be directly used without transformation.
///
/// # Example:
/// ```rust
/// impl_from_error!(DatabaseErrorType, InternalError);
/// ```
macro_rules! impl_from_error {
    ($source_type:ty, $variant:ident) => {
        impl From<$source_type> for crate::error::AppError {
            fn from(value: $source_type) -> Self {
                Self::$variant(value)
            }
        }
    };
}

/// Macro to implement `From<$source_type>` for `AppError::BadRequest` variant.
/// This macro is designed for errors that should be returned with a 400 status code and a detailed error message.
/// It creates an `ErrorResponse` with a `Bad Request` status, setting the `kind` and `message` based on the source error.
///
/// # Example:
/// ```rust
/// impl_bad_request_app_error!(ValidationErrorType);
/// ```
macro_rules! impl_bad_request_app_error {
    ($source_type:ty) => {
        impl From<$source_type> for crate::error::AppError {
            fn from(value: $source_type) -> Self {
                let kind: &'static str = value.clone().into();
                Self::BadRequest(crate::error::ErrorResponse {
                    status: 400,
                    error: "Bad Request".to_string(),
                    kind: kind.to_string(),
                    message: value.to_string(),
                })
            }
        }
    };
}

pub(crate) use impl_bad_request_app_error;
pub(crate) use impl_from_error;
pub(crate) use impl_from_error_to_string;

/// Represents a standardized error response returned by the API.
///
/// This struct is designed to provide structured error information to clients,
/// including an HTTP status code, an error identifier, a category describing
/// the error type, and a user-friendly error message.
#[derive(Debug, Clone, utoipa::ToSchema, serde::Serialize)]
pub struct ErrorResponse {
    /// The HTTP status code associated with the error.
    pub status: u16,

    /// A brief string identifying the type of error.
    pub error: String,

    /// The category or kind of the error, often used to classify error types.
    pub kind: String,

    /// A descriptive message providing additional details about the error.
    pub message: String,
}

/// Enum representing application-level errors.
///
/// The `AppError` enum provides general error variants for various application-wide issues,
/// such as missing resources, access restrictions, bad requests, or internal problems.
pub enum AppError {
    /// Indicates that the specified resource was not found, with details provided in the string.
    NotFound(String),

    /// Represents a forbidden access attempt, typically due to insufficient permissions.
    Forbidden(String),

    /// Indicates an internal error, with a message describing the specific issue.
    InternalError(String),

    /// Represents a bad request error with an associated error response.
    BadRequest(ErrorResponse),

    /// Indicates that the request was successful but no content is available.
    NoContent,
}

impl_from_error_to_string!(sea_orm::DbErr, InternalError);
impl_from_error_to_string!(s3::error::S3Error, InternalError);
impl_from_error!(ErrorResponse, BadRequest);

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Forbidden(err) => (StatusCode::FORBIDDEN, err).into_response(),
            Self::NotFound(_err) => (
                StatusCode::NOT_FOUND,
                "The ressource you've asked doesn't exist / was not found",
            )
                .into_response(),
            Self::BadRequest(err) => (StatusCode::BAD_REQUEST, axum::Json(err)).into_response(),
            Self::InternalError(err) => {
                log::warn!("{err}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Sorry but something unexpected happened, if this continue please contact the Admin").into_response()
            }
            Self::NoContent => (StatusCode::NO_CONTENT, "You're not logged in").into_response(),
        }
    }
}
