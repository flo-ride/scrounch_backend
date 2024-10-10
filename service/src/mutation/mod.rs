//! Mutation services for the `scrounch_backend` application.
//!
//! This module defines mutation-related services and functions responsible for modifying
//! data in the application's data sources. Mutation services handle operations such as creating,
//! updating, and deleting records in the database. They encapsulate the logic for applying changes
//! to the data, ensuring that mutations are executed correctly and consistently across the application.

mod product;
mod user;

pub struct Mutation;
