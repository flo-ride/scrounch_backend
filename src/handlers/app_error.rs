//! Application-specific error handling for request handlers.
//!
//! This module defines custom error types and utilities to handle errors in the `scrounch_backend` application,
//! specifically for use in HTTP request handlers. These error types are designed to provide more
//! meaningful error messages and HTTP responses when errors occur during request processing.

use crate::error::AppError;
use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Converts an `AppError` into an HTTP response.
///
/// This implementation ensures that whenever an `AppError` occurs in a handler, it will be
/// automatically converted into a meaningful HTTP response with an appropriate status code.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Forbidden => (StatusCode::FORBIDDEN, "You cannot view/do this".to_string()),
            AppError::NotFound(value) => {
                tracing::warn!("NotFound: {value}");
                (
                    StatusCode::NOT_FOUND,
                    format!("Sorry, but cannot found the ressource you're asking for: {value}"),
                )
            }
            AppError::MissingOption(value) => {
                tracing::warn!("MissingOption: {value}");
                (
                    StatusCode::BAD_REQUEST,
                    format!("Your request is missing something: {value}"),
                )
            }
            AppError::BadOption(value) => {
                tracing::warn!("BadOption: {value}");
                (
                    StatusCode::BAD_REQUEST,
                    format!("Some parameter you've given is not correctly formatted: {value}"),
                )
            }
            _ => {
                tracing::error!("{self}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unhandled internal error".to_string(),
                )
            }
        }
        .into_response()
    }
}
