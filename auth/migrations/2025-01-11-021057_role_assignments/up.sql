CREATE TABLE role_assignments (
    rbac_id UUID PRIMARY KEY REFERENCES rbac_profiles(id) ON DELETE CASCADE,
    role_id INT NOT NULL REFERENCES roles(id) ON DELETE CASCADE
);