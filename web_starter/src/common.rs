use crate::serde::deserialize_number;
use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_SIZE: u64 = 15;

const DEFAULT_MIN_SIZE: u64 = 1;
const DEFAULT_MAX_SIZE: u64 = 100;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = DEFAULT_MIN_SIZE, message = "Page must be greater than 0"))]
    #[serde(default = "default_page", deserialize_with = "deserialize_number")]
    pub page: u64,

    #[validate(range(min = DEFAULT_MIN_SIZE, max = DEFAULT_MAX_SIZE, message = "Page size must be greater than 0"))]
    #[serde(default = "default_size", deserialize_with = "deserialize_number")]
    pub size: u64,
}

fn default_page() -> u64 {
    DEFAULT_PAGE
}
fn default_size() -> u64 {
    DEFAULT_SIZE
}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub page: u64,
    pub size: u64,
    pub total: u64,
    pub items: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(page: u64, size: u64, total: u64, items: Vec<T>) -> Self {
        Self {
            page,
            size,
            total,
            items,
        }
    }

    pub fn from_pagination_params(params: PaginationParams, total: u64, items: Vec<T>) -> Self {
        Self {
            page: params.page,
            size: params.size,
            total,
            items,
        }
    }
}
