CREATE TABLE role_assignments (
    rbac_id UUID REFERENCES rbac_profiles(id) ON DELETE CASCADE,
    role_id INT REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (rbac_id, role_id)
);