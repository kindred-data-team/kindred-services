-- Your SQL goes here
CREATE TABLE IF NOT EXISTS 'service_appointments' (
    'id' INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    'patient_id' INT NOT NULL,
    'clinic_id' INT NOT NULL,
    'service_variant_id' INT NOT NULL,
    'service_time_slot_id' INT NOT NULL,
    'dosage_number' INT DEFAULT 1,
    'doctor_id' INT DEFAULT NULL,
    'status' VARCHAR(255) DEFAULT NULL,
    'created_by' INT DEFAULT NULL,
    'updated_by' INT DEFAULT NULL,
    'created_at' TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    'updated_at' TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY ('id'),
    FOREIGN KEY ('clinic_id') REFERENCES 'clinics' ('id') ON DELETE CASCADE,
    FOREIGN KEY ('service_time_slot_id') REFERENCES 'clinic_service_time_slots' ('id') ON DELETE CASCADE,
    FOREIGN KEY ('service_variant_id') REFERENCES 'health_service_variants' ('id') ON DELETE CASCADE
)
