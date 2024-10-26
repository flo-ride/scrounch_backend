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
#[cfg(feature = "cache")]
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

/// Macro to retrieve multiple cached values from Redis based on a list of keys.
/// The macro first gets the list of keys from Redis and then attempts to retrieve the corresponding values.
/// If all values are found and can be deserialized into the specified model, they are returned as a result.
#[cfg(feature = "cache")]
macro_rules! cache_mget {
    ($conn:expr, $id:expr, $model:ty) => {{
        {
            use fred::interfaces::KeysInterface;
            if let Some(conn) = &$conn.cache_connection {
                if let Ok(values) = conn.get::<serde_json::Value, _>($id.clone()).await {
                    if let Ok(values) = serde_json::from_value::<Vec<String>>(values) {
                        if let Ok(values) =
                            conn.mget::<Vec<fred::serde_json::Value>, _>(values).await
                        {
                            let result = values
                                .iter()
                                .map(|x| serde_json::from_value::<$model>(x.clone()))
                                .collect::<Vec<_>>();
                            if result.iter().all(|x| x.is_ok()) {
                                return Ok(result.into_iter().filter_map(|x| x.ok()).collect());
                            }
                        }
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
#[cfg(feature = "cache")]
macro_rules! cache_set {
    ($conn:expr, $id:expr, $model:expr, $expiration_secs:expr) => {{
        use fred::{interfaces::KeysInterface, types::Expiration};
        if let Ok(value) = serde_json::to_value($model.to_owned()) {
            if let Some(conn) = &$conn.cache_connection {
                let value_str = value.to_string();
                let cache_conn = conn.clone();
                let id = $id;
                tokio::spawn(async move {
                    let _ = cache_conn
                        .set::<fred::bytes::Bytes, _, _>(
                            id,
                            value_str,
                            Some(Expiration::EX($expiration_secs)),
                            None,
                            false,
                        )
                        .await;
                });
            }
        }
    }};
}

/// Macro to store multiple values in Redis with a specified expiration time.
/// The macro first stores a list of keys associated with the objects, and then sets each object with its corresponding key in Redis.
/// This is done asynchronously using `tokio::spawn` to avoid blocking the main thread.
#[cfg(feature = "cache")]
macro_rules! cache_mset {
    ($conn:expr, $id:expr, $result:expr, $expiration_secs:expr, $prefix:expr) => {{
        {
            use fred::{interfaces::KeysInterface, types::Expiration};
            if let Some(conn) = &$conn.cache_connection {
                let cache_conn = conn.clone();
                let id = $id.to_owned();
                let result = $result.to_owned();
                let prefix = $prefix.to_owned();
                tokio::spawn(async move {
                    if let Ok(value) = serde_json::to_value(
                        result
                            .iter()
                            .map(|x| format!("{prefix}{}", x.id))
                            .collect::<Vec<_>>(),
                    ) {
                        let _ = cache_conn
                            .set::<fred::bytes::Bytes, _, _>(
                                id,
                                value.to_string(),
                                Some(Expiration::EX($expiration_secs)),
                                None,
                                false,
                            )
                            .await;
                    }

                    for user in result {
                        let key = format!("{prefix}{}", user.id);
                        if let Ok(value) = serde_json::to_string(&user) {
                            let _ = cache_conn
                                .set::<fred::bytes::Bytes, _, _>(
                                    key,
                                    value,
                                    Some(Expiration::EX($expiration_secs)),
                                    None,
                                    false,
                                )
                                .await;
                        }
                    }
                });
            }
        }
    }};
}

/// A macro for deleting multiple keys in Redis that start with a given prefix.
///
/// This macro retrieves all keys matching the specified prefix from Redis and deletes them.
/// It is useful for clearing a specific group of cached items, such as all users or products.
///
/// # Features
/// - Requires the `cache` feature to be enabled.
/// - Uses Redis to delete keys based on the provided prefix.
///
/// # Parameters
/// - `$conn`: The cache connection, expected to be an instance with a Redis connection.
/// - `$prefix`: The prefix of the keys to delete. All keys starting with this prefix will be removed.
#[cfg(feature = "cache")]
macro_rules! cache_mdel {
    ($conn:expr, $prefix:expr) => {{
        use fred::interfaces::KeysInterface;
        use fred::types::Scanner;
        use futures::stream::TryStreamExt;

        if let Some(conn) = &$conn.cache_connection {
            let cache_conn = conn.next().clone();
            let prefix = $prefix.to_owned();

            tokio::spawn(async move {
                let mut scan_stream = cache_conn.scan(format!("{}*", prefix), Some(10), None);

                while let Some(mut page) = scan_stream.try_next().await.unwrap_or(None) {
                    if let Some(keys) = page.take_results() {
                        let _ = cache_conn.del::<fred::bytes::Bytes, _>(keys).await;
                    }
                }
            });
        }
    }};
}

pub(crate) use cache_get;
pub(crate) use cache_mdel;
pub(crate) use cache_mget;
pub(crate) use cache_mset;
pub(crate) use cache_set;
