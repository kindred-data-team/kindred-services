use serde::{Deserialize, Serialize};
use validator::Validate;

use super::response_model::Meta;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub product_type_id: usize
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct HealthService {
    pub id: i32,
    pub product_type_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub details: Option<String>,
    pub for_whom: Option<String>,
    pub shopify_id: Option<String>,
    pub shopify_sku: Option<String>,
    pub shopify_variant_id: Option<String>,
    pub image_url: Option<String>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateHealthService {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(range(min = 1, message = "Product type id is required"))]
    pub product_type_id: i32,
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
    #[validate(length(min = 1, message = "Details is required"))]
    pub details: String,
    pub shopify_id: Option<String>,
    pub shopify_sku: Option<String>,
    pub shopify_variant_id: Option<String>,
    pub for_whom: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct EditHealthService {
    pub name: Option<String>,
    pub product_type_id: Option<i32>,
    pub details: Option<String>,
    pub shopify_id: Option<String>,
    pub shopify_sku: Option<String>,
    pub shopify_variant_id: Option<String>,
    pub for_whom: Option<String>,
    pub description: Option<String>
}
#[derive(Debug, Serialize)]
pub struct PaginatedHealthServices {
    pub data: Vec<HealthService>,
    pub meta: Meta
}