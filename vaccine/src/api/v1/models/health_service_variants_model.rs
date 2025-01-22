use serde::{Deserialize, Serialize};
use validator::Validate;

use super::response_model::Meta;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub service_id: usize
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct HealthServiceVariant {
    pub id: i32,
    pub name: Option<String>,
    pub service_id: Option<i32>,
    pub number_of_dose: Option<i32>,
    pub shopify_id: Option<String>,
    pub shopify_sku: Option<String>,
    pub shopify_variant_id: Option<String>,
    pub price: Option<i32>
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateHealthServiceVariant {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(range(min = 1, message = "Service_id is required"))]
    pub service_id: i32,
    pub number_of_dose: Option<i32>,
    pub shopify_id: Option<String>,
    pub shopify_sku: Option<String>,
    pub shopify_variant_id: Option<String>,
    pub price: Option<f32>
}

#[derive(Serialize, Deserialize, Validate)]
pub struct EditHealthServiceVariant {
    pub name: Option<String>,
    pub service_id: Option<i32>,
    pub number_of_dose: Option<i32>,
    pub shopify_id: Option<String>,
    pub shopify_sku: Option<String>,
    pub shopify_variant_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedHealthServiceVariants {
    pub data: Vec<HealthServiceVariant>,
    pub meta: Meta
}