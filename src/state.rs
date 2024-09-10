//! This module defines the `AppState` struct, which holds shared state for the `scrounch_backend` application.

use crate::cli::Arguments;

/// Global application state.
///
/// This struct holds the state shared across the entire `scrounch_backend` application.
#[derive(Clone)]
pub struct AppState {
    pub arguments: Arguments,
}

/// Allows Axum to extract the `Arguments` from `AppState`.
///
/// This implementation enables Axum's request handlers to extract the command-line arguments
/// from the shared application state using the `FromRef` trait.
impl axum::extract::FromRef<AppState> for Arguments {
    fn from_ref(state: &AppState) -> Self {
        state.arguments.clone()
    }
}
