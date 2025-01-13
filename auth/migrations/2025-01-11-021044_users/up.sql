CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR,
    email VARCHAR UNIQUE NOT NULL,
    is_regular BOOLEAN DEFAULT TRUE,
    is_doctor BOOLEAN DEFAULT FALSE,
    ops_team BOOLEAN DEFAULT FALSE,
    email_verified_at TIMESTAMP,
    password VARCHAR,
    google_id VARCHAR,
    apple_id VARCHAR,
    remember_token VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    rbac_id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid() 
);