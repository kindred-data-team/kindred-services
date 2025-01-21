-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `health_service_variants` (
  `id` INT AUTO_INCREMENT PRIMARY KEY,
  `name` VARCHAR(255) UNIQUE NOT NULL,
  `service_id` INT NOT NULL,
  `number_of_dose` INT DEFAULT 1,
  `price` INT DEFAULT 1,
  `shopify_id` VARCHAR(255) DEFAULT NULL,
  `shopify_sku` VARCHAR(255) DEFAULT NULL,
  `shopify_variant_id` VARCHAR(255) DEFAULT NULL,
  `created_by` INT DEFAULT NULL,
  `updated_by` INT DEFAULT NULL,
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  FOREIGN KEY (`service_id`) REFERENCES `health_services` (`id`) ON DELETE CASCADE
);
