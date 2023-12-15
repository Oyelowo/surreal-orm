use surreal_models::migrations::Resources;
use surreal_orm::migrator::{MigrationConfig, RollbackOptions, UpdateStrategy};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

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

    // ONE WAY MIGRATIONS
    let files_config = MigrationConfig::new().make_strict();

    let one_way = files_config.custom_path("migrations-oneway").one_way();
    // Comment out this line to generate oneway migrations
    // To be used from cli
    one_way
        .generate_migrations("migration_name_example", Resources)
        .await
        .unwrap();

    // Run normal non-embedded pending migrations in migration directory
    one_way
        .run_pending_migrations(db.clone(), UpdateStrategy::Latest)
        .await
        .unwrap();

    // TWO WAY MIGRATIONS
    let two_way = files_config.custom_path("migrations-twoway").two_way();

    // GENERATE MIGRATIONS
    // comment out this line to generate twoway migrations
    // To be used from cli
    two_way
        .generate_migrations("migration_name_example", Resources)
        .await
        .unwrap();
    two_way
        .run_down_migrations(db.clone(), RollbackOptions::default())
        // .run_down_migrations(db.clone(), RollbackOptions::default().strategy(RollbackStrategy::Number(4)))
        // .run_down_migrations(
        //     db.clone(),
        //     RollbackOptions::default().strategy(RollbackStrategy::Till("name".to_string().try_into().unwrap())),
        // )
        .await
        .unwrap();

    // Run normal non-embedded pending migrations in migration directory
    two_way
        .run_up_pending_migrations(db.clone(), UpdateStrategy::Latest)
        .await
        .unwrap();
}
