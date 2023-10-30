use clap::Parser;
use surreal_models::migrations::Resources;
use surreal_orm::migrator::{cli, MigrationConfig, RollbackStrategy};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Connection, Surreal};

async fn initialize_db() -> Surreal<surrealdb::engine::remote::ws::Client> {
    let db = Surreal::new::<Ws>("localhost:8000")
        .await
        .expect("Failed to connect to db");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to signin");
    db.use_ns("test").use_db("test").await.unwrap();
    db
}

#[tokio::main]
async fn main() {
    let db = initialize_db().await;
    cli::migration_cli(db, Resources).await;
}
