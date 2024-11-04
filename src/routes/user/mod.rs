//! User-related routes and handlers.
//!
//! This module defines routes and handlers that manage user-related functionality
//! in the `scrounch_backend` application. These routes typically handle operations
//! such as retrieving user information, updating user profiles, and other user-centric
//! tasks.
use utoipa_axum::{router::OpenApiRouter, routes};

pub mod edit;
pub mod get;
pub mod me;

pub fn router() -> OpenApiRouter<crate::state::AppState> {
    OpenApiRouter::new()
        .routes(routes!(get::get_user))
        .routes(routes!(get::get_all_users))
        .routes(routes!(edit::edit_user))
}
