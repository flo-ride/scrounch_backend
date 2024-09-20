//! Custom extractors for the `scrounch_backend` application.
//!
//! This module contains custom extractors that are used to extend the functionality
//! of `axum` in handling and processing requests. Extractors are responsible for
//! parsing incoming request data, such as headers, query parameters, or authentication
//! details, and making that data available to route handlers.

pub mod oidc_user;
pub mod user;
