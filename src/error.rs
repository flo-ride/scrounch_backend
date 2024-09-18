//! This module defines the `AppError` struct, which define common Error for the `scrounch_backend` application.

/// Custom error type for the `scrounch_backend` application.
///
/// This enum defines the error types used within the application, allowing for easy handling
/// of different error scenarios. Currently, it supports:
/// - `DatabaseError`: Represents an error occurring while interacting with the database.
///
/// The `AppError` type is used throughout the application to simplify error management and
/// provide more meaningful error messages when something goes wrong.
pub enum AppError {
    DatabaseError,
}

impl From<sea_orm::DbErr> for AppError {
    fn from(_value: sea_orm::DbErr) -> Self {
        Self::DatabaseError
    }
}
