use surreal_models::migrations::Resources;
use surreal_orm::migrator::{
    self, embed_migrations, MigrationConfig, RollbackOptions, RollbackStrategy, UpdateStrategy,
};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

// Embed migrations as constant
const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("migrations-oneway", one_way, strict);

const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay =
    embed_migrations!("migrations-twoway");

#[tokio::main]
async fn main() {
    let db = initialize_db().await;

    // ONE WAY MIGRATIONS
    let files_config = MigrationConfig::new().make_strict();

    let one_way = files_config
        .clone()
        .set_custom_path("migrations-oneway")
        .one_way();
    // Comment out this line to generate oneway migrations
    // To be used from cli
    one_way
        .generate_migrations(&"migration_name_example".to_string(), Resources)
        .await
        .unwrap();

    // Run normal non-embedded pending migrations in migration directory
    one_way
        .run_pending_migrations(db.clone(), UpdateStrategy::Latest)
        .await
        .unwrap();

    one_way
        .run_embedded_pending_migrations(
            db.clone(),
            MIGRATIONS_ONE_WAY,
            migrator::UpdateStrategy::Latest,
        )
        .await
        .unwrap();

    // TWO WAY MIGRATIONS
    let two_way = files_config.set_custom_path("migrations-twoway").two_way();

    // GENERATE MIGRATIONS
    // comment out this line to generate twoway migrations
    // To be used from cli
    two_way
        .generate_migrations(&"migration_name_example".to_string(), Resources)
        .await
        .unwrap();

    two_way
        .run_down_migrations(
            db.clone(),
            RollbackOptions::new().strategy(RollbackStrategy::Previous),
        )
        // .run_down_migrations(db.clone(), RollbackStrategy::Number(4))
        // .run_down_migrations(
        //     db.clone(),
        // RollbackOptions::new().strategy(RollbackStrategy::Till("name".to_string().try_into().unwrap())),
        //     RollbackStrategy::Till("name".to_string().try_into().unwrap()),
        // )
        .await
        .unwrap();

    // Run normal non-embedded pending migrations in migration directory
    two_way
        .run_up_pending_migrations(db.clone(), migrator::UpdateStrategy::Latest)
        .await
        .unwrap();

    two_way
        .run_up_embedded_pending_migrations(db.clone(), MIGRATIONS_TWO_WAY, UpdateStrategy::Latest)
        .await
        .unwrap();
}

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
