#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Pagination {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}
