use utoipa_axum::{router::OpenApiRouter, routes};

pub mod delete;
pub mod edit;
pub mod get;
pub mod new;

pub fn router() -> OpenApiRouter<crate::state::AppState> {
    OpenApiRouter::new()
        .routes(routes!(get::get_refill))
        .routes(routes!(get::get_all_refills))
        .routes(routes!(new::post_new_refill))
        .routes(routes!(edit::edit_refill))
        .routes(routes!(delete::delete_refill))
}
