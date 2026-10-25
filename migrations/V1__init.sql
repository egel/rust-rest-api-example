CREATE DATABASE IF NOT EXIST rust_rest_api;

CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name varchar(255) NOT NULL,
    is_done BOOLEAN NOT NULL DEFAULT false,
    due_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);