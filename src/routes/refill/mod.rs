use axum::routing::{delete, get, post, put};

pub mod get;
pub mod new;

pub fn router() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/:id", get(get::get_refill))
        .route("/", get(get::get_all_refills))
        .route("/", post(new::post_new_refill))
}
