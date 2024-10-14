use axum::routing::{delete, get, post, put};

pub mod delete;
pub mod edit;
pub mod get;
pub mod new;

pub fn router() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/:id", get(get::get_product))
        .route("/", get(get::get_all_products))
        .route("/", post(new::post_new_product))
        .route("/:id", put(edit::edit_product))
        .route("/:id", delete(delete::delete_product))
}
