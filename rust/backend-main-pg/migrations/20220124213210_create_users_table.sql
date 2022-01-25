-- Add migration script here

CREATE TABLE IF NOT EXISTS users
(
    id         uuid PRIMARY KEY NOT NULL,
    first_name VARCHAR(128) NOT NULL,
    last_name  VARCHAR(128) NOT NULL,
    email      VARCHAR(128) NOT NULL UNIQUE
);
