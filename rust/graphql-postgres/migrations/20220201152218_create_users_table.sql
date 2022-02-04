-- Add migration script here
-- Add migration script here
create type user_role as enum ('user', 'admin');

create table if not exists "users"
(
    id          uuid primary key default uuid_generate_v1mc(),
    created_at  timestamptz not null default timezone('utc', now()),
    updated_at  timestamptz,
    deleted_at  timestamptz default null,
    username    text collate "case_insensitive" unique not null unique,
    first_name  text not null,
    last_name   text not null,
    age         smallint not null,
    email       text collate "case_insensitive" not null unique,
    role        user_role    not null,
    disabled    text,
    last_login  timestamptz default null
);

-- Set updated_at
SELECT trigger_updated_at('"users"');