use serde_derive::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Vaccine {
    pub id: Option<i32>,
    pub name: String,
    pub brand: String,
    pub details: String,
    pub for_whom: String,
    pub price: i32,
    pub number_of_dose: i32,
    pub code: String,
    pub shopify_id: String,
    pub shopify_sku: String,
    pub shopify_variant_id: String,
    
    
    pub variant_id: Option<String>,  
    pub created_at: Option<chrono::DateTime<Utc>>,  
    pub updated_at: Option<chrono::DateTime<Utc>>,  
}

impl Vaccine {
    pub fn new(
        id: Option<i32>,
        name: String,
        brand: String,
        details: String,
        for_whom: String,
        price: i32,
        number_of_dose: i32,
        code: String,
        shopify_id: String,
        shopify_sku: String,
        shopify_variant_id: String,
        variant_id: Option<String>,
        created_at: Option<chrono::DateTime<Utc>>,
        updated_at: Option<chrono::DateTime<Utc>>,
    ) -> Vaccine {
        Vaccine {
            id,
            name,
            brand,
            details,
            for_whom,
            price,
            number_of_dose,
            code,
            shopify_id,
            shopify_sku,
            shopify_variant_id,
            variant_id,
            created_at,
            updated_at,
        }
    }
}