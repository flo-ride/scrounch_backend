//! Defines the request models for interactions with the `entity` module.
//!
//! This module contains various structs that represent the data expected in
//! incoming requests. These request models help validate and structure
//! information received from clients before it is processed and stored in
//! the application's database.
//!
//! These models may represent data for creating, updating, or querying
//! records related to different entities in the system, like products, users,
//! or locations, depending on the application's functionality.

pub mod product;
pub mod user;
