-- Your SQL goes here
CREATE TABLE IF NOT EXISTS 'health_services' (
    'id' INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    'product_type_id' INT NOT NULL,
    'name' VARCHAR(255) NOT NULL UNIQUE,
    'description' TEXT,
    'details' TEXT,
    'for_whom' VARCHAR(255),
    'price' FLOAT DEFAULT NULL,
    'shopify_id' VARCHAR(255),
    'shopify_sku' VARCHAR(255),
    'shopify_variant_id' VARCHAR(255),
    'created_by' INT DEFAULT NULL,
    'updated_by' INT DEFAULT NULL,
    'created_at' TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    'updated_at' TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY ('product_type_id') REFERENCES 'product_types' ('id') ON DELETE CASCADE
);