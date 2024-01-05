// use pretty_env_logger;
use surreal_models::migrations::{Resources, ResourcesV2, ResourcesV31};
use surreal_orm::migrator::config::{DatabaseConnection, UrlDb};
use surreal_orm::migrator::{Migrator, Mode, RealPrompter};
use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

async fn _initialize_db() -> Surreal<Any> {
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
    let db_conn_config = DatabaseConnection::builder()
        .user("root".into())
        .pass("root".into())
        .db("test".into())
        .ns("test".into())
        .url(UrlDb::Others("http://localhost:8000".into()))
        .build();

    // Migrator::builder()
    //     .db_connection(fdfdf)
    //     .run(Resources)
    //     .await;
    // let x = Migrator::builder().db_connection().;
    Migrator::run(ResourcesV31).await;

    // let mut migrator = Migrator::builder()
    //     .verbose(3)
    //     .db_connection(db_conn_config)
    //     .mode(Mode::Strict)
    //     .build()
    //     .run_fn(ResourcesV2, RealPrompter)
    //     .await;
}
