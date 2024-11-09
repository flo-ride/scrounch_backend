//! Service layer of the `scrounch_backend` application.
//!
//! This module defines the service layer, which includes business logic and interactions
//! between different parts of the application. Services encapsulate operations and coordinate
//! tasks such as data retrieval, processing, and manipulation. They act as an intermediary
//! between request handlers and the underlying data sources or other services.

mod r#macro;
mod mutation;
mod query;
pub mod s3;
mod utils;

pub use mutation::Mutation;
pub use query::Query;
pub use utils::every_condition;

pub use sea_orm;

/// Wrapper struct for managing both neaded connections for database operation
///
/// The `Connection` struct encapsulates the connections used in the `scrounch_backend` application.
/// It provides a way to access both the database connection and caching connection throughout the
/// application, ensuring consistent management of these resources.
pub struct Connection {
    pub db_connection: sea_orm::DbConn,
    #[cfg(feature = "cache")]
    pub cache_connection: Option<fred::prelude::RedisPool>,
}
