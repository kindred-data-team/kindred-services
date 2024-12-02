use serde_derive::{Deserialize, Serialize};

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
}

// Implement methods for the Vaccine struct
impl Vaccine {
    // Constructor method to create a new Vaccine instance
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
        }
    }

    // A method to get a summary of the vaccine information
   
}