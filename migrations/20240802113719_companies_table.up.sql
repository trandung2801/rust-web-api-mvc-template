-- Add up migration script here
CREATE TABLE IF NOT EXISTS companies (
    id serial PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    address TEXT,
    description TEXT,
    is_delete BOOLEAN NOT NULL ,

    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);

