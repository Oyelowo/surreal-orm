pub use sea_orm_migration::prelude::*;

mod m20220101_000001_setup;
mod m20220101_000002_create_users_table;
mod m20220101_000003_create_posts_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Box::new(m20220101_000001_setup::Migration), //Commented out for now. Test how the seaorm handles things without it
            Box::new(m20220101_000002_create_users_table::Migration),
            Box::new(m20220101_000003_create_posts_table::Migration),
        ]
    }
}
