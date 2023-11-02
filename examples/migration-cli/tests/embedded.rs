use surreal_models::migrations::Resources;
use surreal_orm::migrator::{self, embed_migrations, MigrationConfig};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

// Embed migrations as constant
const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("tests/migrations-oneway", one_way, strict);

const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay =
    embed_migrations!("tests/migrations-twoway", two_way, strict);

#[test]
fn test_embedded() {
    assert_eq!(MIGRATIONS_ONE_WAY.get_migrations().len(), 2);
    assert_eq!(MIGRATIONS_TWO_WAY.get_migrations().len(), 1);

    let migs = MIGRATIONS_ONE_WAY.to_migrations_one_way().unwrap();
    assert_eq!(migs.len(), 2);
    // check the meta data
    assert_eq!(migs[0].name, "20231029202315_create_new_stuff");
    assert_eq!(migs[0].timestamp, 20231029202315);
    insta::assert_snapshot!(migs[0].content);
    assert_eq!(migs[1].name, "20231029224601_create_new_stuff");
    assert_eq!(migs[1].timestamp, 20231029224601);
    assert_eq!(migs.len(), 2);
    assert_eq!(
        migs[1].content,
        "DEFINE FIELD labels ON planet TYPE array;\nUPDATE planet SET labels = tags;\nREMOVE FIELD tags ON TABLE planet;"
    );

    let migs = MIGRATIONS_TWO_WAY.to_migrations_two_way().unwrap();
    assert_eq!(migs.len(), 1);

    // check the meta data
    assert_eq!(migs[0].name, "20231030025711_migration_name_example");
    assert_eq!(migs[0].timestamp, 20231030025711);
    insta::assert_snapshot!(migs[0].up);
    insta::assert_snapshot!(migs[0].down);
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

#[tokio::test]
async fn test_migrations() {
    // let db = initialize_db().await;
    //
    // // ONE WAY MIGRATIONS
    // let files_config = MigrationConfig::new().make_strict();
    //
    // let one_way = files_config.custom_path("migrations-oneway").one_way();
    // // Comment out this line to generate oneway migrations
    // // To be used from cli
    // one_way
    //     .generate_migrations("migration_name_example", Resources)
    //     .await
    //     .unwrap();
    //
    // // Run normal non-embedded pending migrations in migration directory
    // one_way.run_pending_migrations(db.clone()).await.unwrap();
    //
    // // TWO WAY MIGRATIONS
    // let two_way = files_config.custom_path("migrations-twoway").two_way();
    //
    // // GENERATE MIGRATIONS
    // // comment out this line to generate twoway migrations
    // // To be used from cli
    // two_way
    //     .generate_migrations("migration_name_example", Resources)
    //     .await
    //     .unwrap();
    // // two_way
    // //     .rollback_migrations(RollbackStrategy::Latest, db.clone())
    // //     // .rollback_migrations(RollbackStrategy::ByCount(4), db.clone())
    // //     // .rollback_migrations(
    // //     //     RollbackStrategy::UntilMigrationFileName("name".to_string().try_into().unwrap()),
    // //     //     db.clone(),
    // //     // )
    // //     .await
    // //     .unwrap();
    //
    // // Run normal non-embedded pending migrations in migration directory
    // two_way.run_pending_migrations(db.clone()).await.unwrap();
}
