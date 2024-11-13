//! Utility functions and helpers for the `extractor` module.
//!
//! This module provides a set of utility functions and types that aid in common tasks across the `extractor`
//! module, such as data formatting, validation, or conversions. Each utility function is designed to be
//! reusable across various parts of the codebase, ensuring consistency and reducing redundancy.

/// Struct representing the frontend URL of the application.
///
/// `FrontendUrl` encapsulates the URL as a string, providing a dedicated type
/// to handle frontend URL-related operations or configurations.
#[derive(Debug, Clone, PartialEq)]
pub struct FrontendUrl(pub String);

/// Represents configuration parameters for accessing and filtering Sma API resources.
///
/// The `SmaParams` struct is used to store optional settings that configure
/// how requests to the Sma API are constructed, including API access, base URL,
/// and category filtering.
#[derive(Debug, Clone, PartialEq)]
pub struct SmaParams {
    /// Optional base URL for the Sma API.
    pub url: Option<String>,

    /// Optional API key for authenticating requests.
    pub api_key: Option<String>,

    /// Optional list of category IDs to filter API responses by specific categories.
    pub categories: Option<Vec<String>>,
}
