//! This module contains the extractors related to user profile management.
//!
//! It provides functionality for retrieving and validating profile-related data
//! from incoming requests, simplifying profile-specific operations in the application.
//! These extractors ensure that profile data is accessible and correctly structured
//! for further processing in handlers.
pub mod admin;
pub mod oidc_user;
pub mod user;
