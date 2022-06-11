use sea_orm_migration::prelude::*;

#[tokio::main]
async fn run_migration() {
    cli::run_cli(migration::Migrator).await;
}
