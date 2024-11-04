#[derive(Debug, Clone, PartialEq, serde::Deserialize, utoipa::IntoParams)]
pub struct Pagination {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}
