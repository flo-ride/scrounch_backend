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
            AppError::Forbidden => (StatusCode::FORBIDDEN, "You cannot view/do this"),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unhandled internal error",
            ),
        }
        .into_response()
    }
}
