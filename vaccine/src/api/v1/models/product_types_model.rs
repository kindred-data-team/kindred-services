use serde::{Deserialize, Serialize};
use chrono::Utc;
use validator::Validate;

use super::response_model::Meta;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ProductType {
    pub id: i32,
    pub name: Option<String>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateProductType {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String
}

#[derive(Serialize, Deserialize, Validate)]
pub struct EditProductType {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String
}

#[derive(Debug, serde::Serialize)]
pub struct PaginatedProductTypes {
    pub data: Vec<ProductType>,
    pub meta: Meta
}