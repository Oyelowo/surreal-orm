
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
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
$$ language plpgsql;

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
$$ language plpgsql;

create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // let sql = "DROP TABLE `cake`";
        // let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        // manager.get_connection().execute(stmt).await.map(|_| ())
        unimplemented!()

    }
}