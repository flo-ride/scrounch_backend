//! File related models

/// Enum representing the different types of files.
///
/// This enum is used to differentiate between various types of file.
/// It is deserialized from lowercase string values.
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all_fields = "lowercase")]
pub enum FileType {
    /// Type for product-related files.
    Product,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Product => write!(f, "product"),
        }
    }
}