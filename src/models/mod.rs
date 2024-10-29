//! Application models for the `scrounch_backend` service.
//!
//! This module defines the core data models used within the `scrounch_backend` application.
//! These models are utilized by various parts of the application for handling and manipulating
//! data. They do not represent database entities but serve as structures for application log
pub mod r#enum;
pub mod file;
pub mod profile;
pub mod request;
pub mod response;
pub mod utils;
