CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    rbac_id UUID NOT NULL REFERENCES rbac_profiles(id) ON DELETE CASCADE,
    auth_user_id SERIAL NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE,
    access_token TEXT
);

SELECT diesel_manage_updated_at('sessions');