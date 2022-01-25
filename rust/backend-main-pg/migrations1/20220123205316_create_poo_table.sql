-- Add migration script here
CREATE TABLE IF NOT EXISTS poos
(
    id         SERIAL PRIMARY KEY NOT NULL,
    content VARCHAR(128)
);
