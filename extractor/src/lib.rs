//! This module defines custom extractors used within the application,
//! providing specialized ways to retrieve and process data from incoming requests.
//!
//! Each extractor here is implemented to simplify request handling and ensure
//! type-safe access to specific request data. Extractors are designed to handle
//! common validation, authorization, or data transformation logic when interacting
//! with the application’s API endpoints.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod profile;
pub mod query;
