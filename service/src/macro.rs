//! Macros for the `service` crate in the `scrounch_backend` application.
//!
//! This module provides macros to simplify operations related to databases and caching, specifically in the context
//! of Sea-ORM for database queries and Redis for caching. These macros help reduce repetitive code by abstracting
//! common patterns in database access, query building, and cache handling.

/// A macro for retrieving cached data from Redis.
///
/// This macro simplifies the process of checking if a value exists in Redis and deserializing it into a specified model type.
/// If the value is found in the cache, it returns the deserialized model; otherwise, it falls back to other logic.
///
/// # Features
/// - Requires the `cache` feature to be enabled.
/// - Uses Redis to retrieve data via the provided cache connection.
/// - Deserializes the cached value into the specified model type using `serde_json`.
///
/// # Parameters
/// - `$conn`: The cache connection, expected to be an instance with a Redis connection.
/// - `$id`: The cache key to retrieve the value from.
/// - `$model`: The type of model to deserialize the cached value into.
///
/// # Usage
/// ```rust
/// cache_get!(conn, id, your_module::Model);
/// ```
#[cfg(feature = "cache")]
#[macro_export]
macro_rules! cache_get {
    ($conn:expr, $id:expr, $model:ty) => {{
        {
            use fred::interfaces::KeysInterface;
            if let Some(conn) = &$conn.cache_connection {
                if let Ok(value) = conn.get::<fred::serde_json::Value, _>($id.clone()).await {
                    if let Ok(model) = serde_json::from_value::<$model>(value) {
                        return Ok(Some(model));
                    }
                }
            }
        }
    }};
}

/// A macro for setting cached data in Redis with an expiration time.
///
/// This macro allows storing a value in Redis for a specified period. It serializes the given model
/// into JSON format and saves it with the provided key and expiration time.
///
/// # Features
/// - Requires the `cache` feature to be enabled.
/// - Serializes the provided model using `serde_json` and stores it in Redis.
/// - Sets the cache entry with an optional expiration time.
///
/// # Parameters
/// - `$conn`: The connection to the Redis cache. It is expected to have a Redis connection available in the provided structure.
/// - `$id`: The cache key under which the value will be stored.
/// - `$model`: The model to be serialized and stored in the cache.
/// - `$expiration_secs`: The expiration time in seconds for the cached value.
///
/// # Usage
/// ```rust
/// cache_set!(conn, id, your_module::Model, 60 * 20);
/// ```
#[cfg(feature = "cache")]
#[macro_export]
macro_rules! cache_set {
    ($conn:expr, $id:expr, $model:expr, $expiration_secs:expr) => {{
        use fred::{interfaces::KeysInterface, types::Expiration};
        if let Ok(value) = serde_json::to_value($model.to_owned()) {
            if let Some(conn) = &$conn.cache_connection {
                let _ = conn
                    .set::<fred::bytes::Bytes, _, _>(
                        $id,
                        value.to_string(),
                        Some(Expiration::EX($expiration_secs)),
                        None,
                        false,
                    )
                    .await
                    .unwrap();
            }
        }
    }};
}
