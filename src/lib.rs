#![warn(missing_docs)]
//! scrounch_backend - Backend for a sales application
//!
//! This library serves as the core of the `scrounch_backend` crate, providing
//! all backend functionality for managing beverage sales. It integrates the Axum
//! framework for handling HTTP requests and response routing.

mod cli;
mod routes;

pub use cli::Arguments;

/// Creates and configures the Axum application.
///
/// This function sets up the Axum app, defines the routes, middleware,
/// and any other necessary configuration for handling HTTP requests.
/// It wires up the backend services such as authentication, database connections,
/// and any other business logic needed to manage the beverage sales system.
pub async fn app() -> axum::Router {
    axum::Router::new()
        .merge(routes::openapi::openapi())
        .route("/status", axum::routing::get(routes::status::get_status))
}
