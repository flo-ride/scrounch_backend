//! User-related routes and handlers.
//!
//! This module defines routes and handlers that manage user-related functionality
//! in the `scrounch_backend` application. These routes typically handle operations
//! such as retrieving user information, updating user profiles, and other user-centric
//! tasks.
use axum::routing::get;

pub mod get;
pub mod me;

pub fn router() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/:id", get(get::get_user))
        .route("/", get(get::get_all_users))
}
