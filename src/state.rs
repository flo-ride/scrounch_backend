//! This module defines the `AppState` struct, which holds shared state for the `scrounch_backend` application.

use crate::cli::Arguments;
use sea_orm::DatabaseConnection;

/// Global application state.
///
/// This struct holds the state shared across the entire `scrounch_backend` application.
#[derive(Clone)]
pub struct AppState {
    pub arguments: Arguments,
    pub db_pool: DatabaseConnection,
    #[cfg(feature = "cache")]
    pub cache_pool: Option<fred::clients::RedisPool>,

    pub s3_bucket: s3::Bucket,
}

/// Allows Axum to extract the `Arguments` from `AppState`.
///
/// This implementation enables Axum's request handlers to extract the command-line arguments
/// from the shared application state using the `FromRef` trait.
impl axum::extract::FromRef<AppState> for Arguments {
    fn from_ref(state: &AppState) -> Self {
        state.arguments.clone()
    }
}

/// Allows Axum to extract the `DbConnection` from `AppState`.
///
/// This implementation enables Axum's request handlers to extract the database connection pool
/// from the shared application state using the `FromRef` trait.
impl axum::extract::FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

/// Allows Axum to extract the `Bucket` from `AppState`.
///
/// This implementation enables Axum's request handlers to extract the s3 connection
/// from the shared application state using the `FromRef` trait.
impl axum::extract::FromRef<AppState> for s3::Bucket {
    fn from_ref(state: &AppState) -> Self {
        state.s3_bucket.clone()
    }
}

impl axum::extract::FromRef<AppState> for service::Connection {
    fn from_ref(state: &AppState) -> Self {
        Self {
            db_connection: state.db_pool.clone(),
            #[cfg(feature = "cache")]
            cache_connection: state.cache_pool.clone(),
        }
    }
}
