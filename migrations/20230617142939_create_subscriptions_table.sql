-- Add migration script here
-- Create Subscriptions table
CREATE TABLE subscriptions (
    id uuid NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL DEFAULT now(),

    PRIMARY KEY (id)
)