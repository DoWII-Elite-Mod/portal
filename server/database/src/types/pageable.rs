use serde::Serialize;

use super::pagination::Pagination;

#[derive(Debug, Serialize)]
pub struct Pageable<T> {
    pub content: Vec<T>,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(rename = "numberOfElements")]
    pub number_of_elements: usize,
}
