use std::collections::BTreeMap;
use std::fs::{self, ReadDir};
use std::io;
use std::path::PathBuf;

use surreal_models::migrations::{Resources, ResourcesV2};
use surreal_orm::migrator::{
    DbInfo, Informational, Migration, MigrationConfig, MigrationFilename, UpdateStrategy,
};
use surreal_orm::statements::{info_for, select};
use surreal_orm::{All, ReturnableSelect, Runnable};
use surrealdb::engine::local::Db;
use surrealdb::{Connection, Surreal};
use tempfile::tempdir;

#[test]
fn test_migration_cli() {
    // TODO
    // cargo run -- generate --name create_user --reversible --migrations-dir ./migrations
    // cargo run -- run --reversible --migrations-dir ./migrations
    // cargo run -- rollback --reversible --migrations-dir ./migrations
    // cargo run -- reset --reversible --migrations-dir ./migrations
    // cargo run -- redo --reversible --migrations-dir ./migrations
    // cargo run -- status --reversible --migrations-dir ./migrations

    // Write tests for the above commands
}
async fn initialize_db() -> Surreal<impl Connection> {
    let db = Surreal::<Db>::new(())
        .await
        .expect("Failed to connect to db");
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
    let _files = fs::read_dir(temp_test_migration_dir).expect_err("Migrations not yet created");

    let one_way = files_config
        .set_custom_path(temp_test_migration_dir.display().to_string())
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
    one_way
        .run_pending_migrations(db.clone(), UpdateStrategy::Latest)
        .await
        .unwrap();

    // Files would now be created
    let files = fs::read_dir(temp_test_migration_dir).unwrap();
    let (files, contents) = get_files_meta(files);
    assert_eq!(files.len(), 1, "Migration not created");
    insta::assert_snapshot!(files.join("\n"));
    insta::assert_snapshot!(contents);

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

fn get_files_meta(files: ReadDir) -> (Vec<String>, String) {
    let mut file_paths: Vec<PathBuf> = files
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    file_paths.sort_by_key(|path| path.file_name().unwrap().to_str().unwrap().to_lowercase());

    let (file_names, contents) = file_paths
        .into_iter()
        .map(|path| {
            let content = fs::read_to_string(&path).unwrap();

            let name: MigrationFilename = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .try_into()
                .unwrap();
            let simple_name = format!("{}.{}", name.simple_name(), name.extension());

            (simple_name, content)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    (file_names, contents.join("\n"))
}

#[tokio::test]
async fn test_twoway_migrations() {
    let db = initialize_db().await;
    let files_config = MigrationConfig::new().make_strict();

    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let test_migration_name = "test_migration";
    let test_migration_namev2 = "test_migration_v2".to_string();
    let _files = fs::read_dir(temp_test_migration_dir).expect_err("Migrations not yet created");

    let two_way = files_config
        .set_custom_path(temp_test_migration_dir.display().to_string())
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

    // FIRST MIGRATION
    // Generate first migration
    two_way
        .generate_migrations(&test_migration_name.to_string(), Resources)
        .await
        .unwrap();

    assert_eq!(
        get_db_info().await.as_ref().unwrap().tables().get_names(),
        vec![] as Vec<String>
    );

    assert_eq!(get_migrations().await.len(), 0);

    // BEFORE GENERATING SECOND MIGRATION
    let files = fs::read_dir(temp_test_migration_dir).unwrap();

    let (files, contents) = get_files_meta(files);

    assert_eq!(files.len(), 2, "New migrations not created");
    insta::assert_snapshot!(files.join("\n\n"));
    insta::assert_snapshot!(contents);

    // SECOND MIGRATION GENERATION
    two_way
        .generate_migrations(&test_migration_namev2, ResourcesV2)
        .await
        .unwrap();
    let files = fs::read_dir(temp_test_migration_dir).unwrap();

    let (files, contents) = get_files_meta(files);

    assert_eq!(files.len(), 4, "Migration not created");
    insta::assert_snapshot!(files.join("\n"));
    insta::assert_snapshot!(contents);

    assert_eq!(
        get_db_info().await.as_ref().unwrap().tables().get_names(),
        vec![] as Vec<String>
    );

    assert_eq!(get_migrations().await.len(), 0);

    // Run normal non-embedded pending migrations in migration directory
    // two_way.run_pending_migrations(db.clone()).await.unwrap();

    // Files would now be created
    // let files = fs::read_dir(&temp_test_migration_dir).unwrap();
    // assert_eq!(files.count(), 4, "Migration not created");
    //
    // let db_info = get_db_info().await.unwrap();
    // assert_eq!(
    //     db_info.tables().get_names(),
    //     vec!["animal", "crop", "eats", "migration", "new_stuff", "planet"]
    // );
    // assert_eq!(get_migrations().await.len(), 2);

    // let mut table_defs = get_db_info()
    //     .await
    //     .unwrap()
    //     .tables()
    //     .get_all_definitions()
    //     .iter()
    //     .map(|q| q.to_string())
    //     .collect::<Vec<_>>();
    //
    // table_defs.sort();
    // insta::assert_snapshot!(table_defs.join("\n"));
    //
    // insta::assert_debug_snapshot!(get_db_info().await.unwrap());
    // let fields_info = |field: String| async {
    //     let fields_info = info_for()
    //         .table(field)
    //         .get_data::<BTreeMap<String, BTreeMap<String, String>>>(db.clone())
    //         .await
    //         .unwrap();
    //
    //     insta::assert_snapshot!(fields_info
    //         .as_ref()
    //         .unwrap()
    //         .values()
    //         .map(|v| v
    //             .keys()
    //             .map(|k| k.to_string())
    //             .collect::<Vec<_>>()
    //             .join(", "))
    //         .collect::<Vec<_>>()
    //         .join("\n"));
    //     insta::assert_debug_snapshot!(fields_info.unwrap());
    // };
    //
    // fields_info("animal".to_string()).await;
    // fields_info("crop".to_string()).await;
    // fields_info("eats".to_string()).await;
    // fields_info("migration".to_string()).await;
    // fields_info("planet".to_string()).await;
    // fields_info("student".to_string()).await;
    //
    // assert_eq!(get_migrations().await.len(), 2);
    // let files = fs::read_dir(&temp_test_migration_dir).unwrap();
    // assert_eq!(files.count(), 4, "Migrations not deleted");
    //
    // two_way
    //     .rollback_migrations(RollbackStrategy::Latest, db.clone())
    //     .await
    //     .unwrap();
    //
    // assert_eq!(get_migrations().await.len(), 1);
    //
    // // 2 files(up/down) Files would now be deleted
    // let files = fs::read_dir(&temp_test_migration_dir).unwrap();
    // assert_eq!(files.count(), 2, "Migrations not deleted");
}
