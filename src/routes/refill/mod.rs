use axum::routing::{delete, get, post, put};

pub mod new;

pub fn router() -> axum::Router<crate::state::AppState> {
    axum::Router::new().route("/", post(new::post_new_refill))
}
