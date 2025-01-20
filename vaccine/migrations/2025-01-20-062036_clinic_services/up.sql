-- Your SQL goes here
CREATE TABLE IF NOT EXISTS 'clinic_services' (
    'id' int NOT NULL AUTO_INCREMENT PRIMARY KEY,
    'clinic_id' int NOT NULL,
    'service_id' int NOT NULL,
    'created_by' int DEFAULT NULL,
    'updated_by' int DEFAULT NULL,
    'created_at' timestamp DEFAULT CURRENT_TIMESTAMP,
    'updated_at' timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    'plato_code' varchar(255) NOT NULL,
    FOREIGN KEY ('clinic_id') REFERENCES 'clinics' ('id') ON DELETE CASCADE
    FOREIGN KEY ('service_id') REFERENCES 'health_services' ('id') ON DELETE CASCADE
);