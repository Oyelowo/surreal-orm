// use pretty_env_logger;
use surreal_models::migrations::Resources;
use surreal_orm::migrator::cli;
use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

async fn initialize_db() -> Surreal<Any> {
    let db = connect("http://localhost:8000").await.unwrap();
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
    // pretty_env_logger::init();
    let _db = initialize_db().await;
    // cli::migration_cli(Resources, Some(db)).await;
    cli::migration_cli(Resources, None).await;
}
