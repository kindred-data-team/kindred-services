CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('permissions');

-- INSERT INTO permissions (path) VALUES 
-- ('/api/rbac');
