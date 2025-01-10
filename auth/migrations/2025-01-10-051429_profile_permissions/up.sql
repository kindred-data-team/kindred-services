CREATE TABLE profile_permissions (
    rbac_id UUID REFERENCES rbac_profiles(id) ON DELETE CASCADE,
    permission_id INT REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (rbac_id, permission_id)
);