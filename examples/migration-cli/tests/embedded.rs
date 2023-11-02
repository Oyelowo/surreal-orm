use std::collections::{BTreeMap, HashMap};
use std::fs;

use surreal_models::migrations::Resources;
use surreal_orm::migrator::{
    self, embed_migrations, DbInfo, Informational, Migration, MigrationConfig, RollbackStrategy,
};
use surreal_orm::statements::{info_for, select};
use surreal_orm::{All, ReturnableSelect, Runnable};
use surrealdb::engine::local::Db;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Connection, Surreal};
use tempfile::tempdir;

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

async fn initialize_db() -> Surreal<impl Connection> {
    // let db = Surreal::new::<Ws>("localhost:8000")
    let db = Surreal::<Db>::new(())
        .await
        .expect("Failed to connect to db");
    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await
    // .expect("Failed to signin");
    db.use_ns("test").use_db("test").await.unwrap();
    db
}

#[tokio::test]
async fn test_oneway_migrations() {
    let db = initialize_db().await;
    // ONE WAY MIGRATIONS
    let files_config = MigrationConfig::new().make_strict();

    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let test_migration_name = "test_migration";
    let _files = fs::read_dir(&temp_test_migration_dir).expect_err("Migrations not yet created");

    let one_way = files_config
        .custom_path(&temp_test_migration_dir.display().to_string())
        .one_way();

    let db_info = || async {
        info_for()
            .database()
            .get_data::<DbInfo>(db.clone())
            .await
            .unwrap()
    };

    assert_eq!(
        db_info().await.as_ref().unwrap().tables().get_names(),
        vec![] as Vec<String>
    );

    // Comment out this line to generate oneway migrations
    // To be used from cli
    one_way
        .generate_migrations(test_migration_name, Resources)
        .await
        .unwrap();

    assert_eq!(
        db_info().await.as_ref().unwrap().tables().get_names(),
        vec![] as Vec<String>
    );

    // Run normal non-embedded pending migrations in migration directory
    one_way.run_pending_migrations(db.clone()).await.unwrap();

    // Files would now be created
    let files = fs::read_dir(&temp_test_migration_dir).unwrap();
    assert_eq!(files.count(), 1, "Migration not created");

    assert_eq!(
        db_info().await.unwrap().tables().get_names(),
        vec!["animal", "crop", "eats", "migration", "planet", "student"],
        "Tables not created"
    );

    let mut table_defs = db_info()
        .await
        .unwrap()
        .tables()
        .get_all_definitions()
        .iter()
        .map(|q| q.to_string())
        .collect::<Vec<_>>();

    table_defs.sort();
    insta::assert_snapshot!(table_defs.join("\n"));

    insta::assert_debug_snapshot!(db_info().await.unwrap());
    let fields_info = |field: String| async {
        let fields_info = info_for()
            .table(field)
            .get_data::<BTreeMap<String, BTreeMap<String, String>>>(db.clone())
            .await
            .unwrap();

        insta::assert_snapshot!(fields_info
            .as_ref()
            .unwrap()
            .values()
            .map(|v| v
                .keys()
                .map(|k| k.to_string())
                .collect::<Vec<_>>()
                .join(", "))
            .collect::<Vec<_>>()
            .join("\n"));
        insta::assert_debug_snapshot!(fields_info.unwrap());
    };

    fields_info("animal".to_string()).await;
    fields_info("crop".to_string()).await;
    fields_info("eats".to_string()).await;
    fields_info("migration".to_string()).await;
    fields_info("planet".to_string()).await;
    fields_info("student".to_string()).await;
}

#[tokio::test]
async fn test_twoway_migrations() {
    let db = initialize_db().await;
    let files_config = MigrationConfig::new().make_strict();

    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let test_migration_name = "test_migration";
    let _files = fs::read_dir(&temp_test_migration_dir).expect_err("Migrations not yet created");

    let two_way = files_config
        .custom_path(&temp_test_migration_dir.display().to_string())
        .two_way();

    let get_db_info = || async {
        info_for()
            .database()
            .get_data::<DbInfo>(db.clone())
            .await
            .unwrap()
    };
    let get_migrations = || async {
        select(All)
            .from(Migration::table_name())
            .return_many::<Migration>(db.clone())
            .await
            .unwrap()
    };

    assert_eq!(
        get_db_info().await.as_ref().unwrap().tables().get_names(),
        vec![] as Vec<String>
    );

    // Comment out this line to generate oneway migrations
    // To be used from cli
    two_way
        .generate_migrations(test_migration_name, Resources)
        .await
        .unwrap();

    assert_eq!(
        get_db_info().await.as_ref().unwrap().tables().get_names(),
        vec![] as Vec<String>
    );

    assert_eq!(get_migrations().await.len(), 0);
    // Run normal non-embedded pending migrations in migration directory
    two_way.run_pending_migrations(db.clone()).await.unwrap();
    assert_eq!(get_migrations().await.len(), 1);
    // assert_eq!(
    //     migrations()
    //         .await
    //         .iter()
    //         .map(|r| r.clone().id)
    //         .collect::<Vec<_>>(),
    //     vec![]
    // );

    // Files would now be created
    let files = fs::read_dir(&temp_test_migration_dir).unwrap();
    assert_eq!(files.count(), 2, "Migration not created");

    assert_eq!(
        get_db_info().await.unwrap().tables().get_names(),
        vec!["animal", "crop", "eats", "migration", "planet", "student"]
    );

    let mut table_defs = get_db_info()
        .await
        .unwrap()
        .tables()
        .get_all_definitions()
        .iter()
        .map(|q| q.to_string())
        .collect::<Vec<_>>();

    table_defs.sort();
    insta::assert_snapshot!(table_defs.join("\n"));

    insta::assert_debug_snapshot!(get_db_info().await.unwrap());
    let fields_info = |field: String| async {
        let fields_info = info_for()
            .table(field)
            .get_data::<BTreeMap<String, BTreeMap<String, String>>>(db.clone())
            .await
            .unwrap();

        insta::assert_snapshot!(fields_info
            .as_ref()
            .unwrap()
            .values()
            .map(|v| v
                .keys()
                .map(|k| k.to_string())
                .collect::<Vec<_>>()
                .join(", "))
            .collect::<Vec<_>>()
            .join("\n"));
        insta::assert_debug_snapshot!(fields_info.unwrap());
    };

    fields_info("animal".to_string()).await;
    fields_info("crop".to_string()).await;
    fields_info("eats".to_string()).await;
    fields_info("migration".to_string()).await;
    fields_info("planet".to_string()).await;
    fields_info("student".to_string()).await;

    two_way
        .rollback_migrations(RollbackStrategy::Latest, db.clone())
        .await
        .unwrap();

    assert_eq!(get_migrations().await.len(), 0);

    // Files would now be deleted
    let files = fs::read_dir(&temp_test_migration_dir).unwrap();
    assert_eq!(files.count(), 0, "Migrations not deleted");
}
