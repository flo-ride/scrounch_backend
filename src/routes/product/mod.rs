use utoipa_axum::{router::OpenApiRouter, routes};

pub mod delete;
pub mod edit;
pub mod get;
pub mod new;

pub fn router() -> OpenApiRouter<crate::state::AppState> {
    OpenApiRouter::new()
        .routes(routes!(get::get_product))
        .routes(routes!(get::get_all_products))
        .routes(routes!(new::post_new_product))
        .routes(routes!(edit::edit_product))
        .routes(routes!(delete::delete_product))
}
