//! Utility routes and helpers for the `scrounch_backend` application.
//!
//! This module contains utility routes and functions that are shared across multiple
//! route handlers in the application. It can be used to define common route-related
//! helpers, middleware, and utilities that simplify the development of route handlers.

pub mod login;
pub mod logout;
pub mod openapi;
pub mod status;
