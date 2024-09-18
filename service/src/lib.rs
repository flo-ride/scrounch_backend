mod mutation;
mod query;

pub use mutation::Mutation;
pub use query::Query;

pub use sea_orm;

/// Wrapper struct for managing both neaded connections for database operation
///
/// The `Connection` struct encapsulates the connections used in the `scrounch_backend` application.
/// It provides a way to access both the database connection and caching connection throughout the
/// application, ensuring consistent management of these resources.
pub struct Connection {
    pub db_connection: sea_orm::DbConn,
}
