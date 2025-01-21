-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `clinic_service_time_slots` (
    `id` INT AUTO_INCREMENT PRIMARY KEY,
    `clinic_id` INT NOT NULL,
    `product_type_id` INT NOT NULL,
    `date` DATE,
    `start_time` TIME,
    `end_time` TIME,
    `is_available` BOOLEAN DEFAULT TRUE,
    `is_virtual` BOOLEAN DEFAULT FALSE,
    `number_of_slots` INT DEFAULT 1,
    `user_id` INT DEFAULT NULL,
    `created_by` INT DEFAULT NULL,
    `updated_by` INT DEFAULT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (`clinic_id`) REFERENCES `clinics` (`id`) ON DELETE CASCADE,
    FOREIGN KEY (`product_type_id`) REFERENCES `product_types` (`id`) ON DELETE CASCADE
);
