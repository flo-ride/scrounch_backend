//! Procedural macro to derive filter query structs for SeaORM entities.
//!
//! This macro simplifies the creation of filter query structures by generating a new struct
//! with fields for various filter operations (equality, inequality, range comparisons) based on the fields
//! of the input struct. It supports customization through attributes.
#![warn(missing_docs)]

mod filter;
mod helper;

/// Derives a filter query struct for the given struct, generating fields for various filter operations.
///
/// This procedural macro generates a filter query struct for SeaORM entities, adding fields to filter based on
/// equality, inequality, and range comparisons (greater than, less than, greater than or equal, less than or equal).
#[proc_macro_derive(DeriveToFilterQuery, attributes(sea_orm))]
pub fn derive_to_filter_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    filter::derive_to_filter_query(input)
}
