use axum::routing::{delete, get, post, put};

pub mod get;
pub mod new;

pub fn router() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/:id", get(get::get_location))
        .route("/", get(get::get_all_locations))
        .route("/", post(new::post_new_location))
}
