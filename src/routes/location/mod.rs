use utoipa_axum::{router::OpenApiRouter, routes};

pub mod delete;
pub mod edit;
pub mod get;
pub mod new;

pub fn router() -> OpenApiRouter<crate::state::AppState> {
    OpenApiRouter::new()
        .routes(routes!(get::get_location))
        .routes(routes!(get::get_all_locations))
        .routes(routes!(new::post_new_location))
        .routes(routes!(edit::edit_location))
        .routes(routes!(delete::delete_location))
}
