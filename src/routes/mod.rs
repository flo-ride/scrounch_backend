//! Module for defining the application's routes.
//!
//! This module serves as the central hub for all HTTP route definitions
//! in the `scrounch_backend` application. It organizes and exports the
//! different route handlers, ensuring that the Axum router can access
//! them easily.
//!
//! # Structure
//! - This file exports submodules that define individual route handlers.
//! - Each submodule is responsible for a specific section of the API
pub mod location;
pub mod product;
pub mod user;
pub mod utils;
