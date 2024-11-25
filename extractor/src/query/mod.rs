//! # Query Module
//!
//! This module contains query-related utilities and functions for the `extractor` component.
//! It includes definitions and helpers for building, executing, and managing queries
//! used throughout the application. The `query` module provides abstractions to
//! simplify data retrieval from the database and enables efficient query construction
//! for specific data needs.

/// Represents pagination parameters for API requests.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, utoipa::IntoParams)]
pub struct Pagination {
    /// The page number to retrieve, starting from 0.
    pub page: Option<u64>,

    /// The number of items to return per page.
    pub per_page: Option<u64>,
}
