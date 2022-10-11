-- Add migration script here

-- This setup 
-- creates function for autogenerating uuid
-- creates function for auto-updating updated_at field which can be useful for audit log.
create extension if not exists "uuid-ossp";
-- select trigger_updated_at('<table name>');
--
-- after a `CREATE TABLE`.
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = timezone('utc', now());
    return NEW;
end;
$$ language plmysql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plmysql;

create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);