use axum::routing::{delete, get, post, put};

pub mod delete;
pub mod edit;
pub mod get;
pub mod new;

pub fn router() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/:id", get(get::get_refill))
        .route("/", get(get::get_all_refills))
        .route("/", post(new::post_new_refill))
        .route("/:id", put(edit::edit_refill))
        .route("/:id", delete(delete::delete_refill))
}
