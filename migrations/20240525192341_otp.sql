CREATE TABLE IF NOT EXISTS otps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    key VARCHAR(8) NOT NULL,
    is_used BOOLEAN NOT NULL,
    user_id UUID REFERENCES users (id) NOT NULL
);