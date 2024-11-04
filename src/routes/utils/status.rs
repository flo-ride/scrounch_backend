//! Status route for health checks.
//!
//! This module defines the `/status` endpoint, which is used to perform
//! health checks on the `scrounch_backend` application. It provides a simple
//! response indicating whether the server is running and healthy.
//!
//! # Endpoint
//! - **GET /status**: Returns a basic success message with a `200 OK` status.
//!
//! # Usage
//! This route can be used by load balancers, uptime monitors, or administrators
//! to ensure that the server is operational.
//!
//! # Example
//! ```bash
//! curl http://localhost:3000/status
//! ```

use super::openapi::MISC_TAG;

/// Returns the status of the server.
///
/// This asynchronous function handles the `/status` endpoint and returns a
/// static string indicating that the server is running. It is commonly used for
/// health checks to confirm that the application is up and operational.
///
/// # Returns
/// A static string `"UP"`, representing the server's status.
///
/// # Endpoint
/// - **GET /status**: Responds with `"UP"` and a `200 OK` status.
#[utoipa::path(
    get,
    path = "/status",
    tag = MISC_TAG,
    responses(
        (status = 200, description = "API is up and functionnal", body = String)
    )
)]
pub async fn get_status() -> &'static str {
    "UP"
}
