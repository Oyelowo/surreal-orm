use surreal_models::migrations::Resources;
use surreal_orm::migrator::cli;
use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

// async fn initialize_db() -> Surreal<surrealdb::engine::remote::ws::Client> {
async fn initialize_db() -> Surreal<Any> {
    // let db = Surreal::new::<Ws>("localhost:8000")
    //     .await
    //     .expect("Failed to connect to db");

    let db = connect("http://localhost:8000").await.unwrap();
    // let db = Surreal::new::<Ws>("localhost:8000")
    //     .await
    //     .expect("Failed to connect to db");
    // db.connect(address)
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to signin");
    db.use_ns("test").use_db("test").await.unwrap();
    db
}

// static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);
// static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
// Can be one of memory, file:<path>, tikv:<addr>, file://<path>, tikv://<addr>
#[tokio::main]
async fn main() {
    let db = initialize_db().await;
    // include example usage as rust doc
    cli::migration_cli(Some(db), Resources).await;
}
