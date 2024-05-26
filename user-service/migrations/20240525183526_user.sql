CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    username VARCHAR(32) UNIQUE NOT NULL,
    password VARCHAR(64) NOT NULL,
    phone VARCHAR(32) NULL,
    email VARCHAR(32) NULL,
    phone_verified BOOLEAN NOT NULL,
    email_verified BOOLEAN NOT NULL,
    is_admin BOOLEAN NOT NULL
);
CREATE INDEX IF NOT EXISTS users_username ON users (username);