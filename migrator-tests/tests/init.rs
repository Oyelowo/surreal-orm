use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use surreal_models::migrations::{Resources, ResourcesV2};
use surreal_orm::migrator::{
    config::{DatabaseConnection, UrlDb},
    Init, Migration, MigrationFilename, Migrator, MockPrompter, Mode, SubCommand,
};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tempfile::tempdir;

fn read_migs_from_dir(path: PathBuf) -> Vec<DirEntry> {
    std::fs::read_dir(path)
        .expect("Failed to read dir")
        .map(|p| p.expect("Failed to read dir2"))
        .collect::<Vec<_>>()
}

fn assert_migration_files_presence_and_format(
    migration_files: Vec<DirEntry>,
    db_migrations: Vec<Migration>,
    test_migration_name: &str,
) {
    let mut migration_files = migration_files.iter().map(|f| f.path()).collect::<Vec<_>>();
    migration_files.sort_by(|a, b| {
        a.file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to get file name")
            .cmp(
                b.file_name()
                    .expect("Failed to get file name")
                    .to_str()
                    .expect("Failed to get file name"),
            )
    });

    for filepath in migration_files.iter() {
        // TODO: Make snapshot tests deterministict
        // let content = fs::read_to_string(&filepath).expect("Failed to read file");
        // insta::assert_display_snapshot!(content);

        let file_name = filepath
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to get file name");
        let file_name =
            MigrationFilename::try_from(file_name.to_string()).expect("Failed to parse file name");

        let timestamp = file_name.timestamp();
        let basename = file_name.basename();
        let extension = file_name.extension();

        // we want to test that the migration file metadata is stored in the db
        // e.g:  the name, timestamp and perhaps checksum?
        // ts_basename.up.surql
        // ts_basename.down.surql
        // ts_basename.sql
        if !db_migrations.is_empty() {
            let found_db_mig = |file_name: MigrationFilename| {
                db_migrations
                    .iter()
                    .find(|m| {
                        let db_name: MigrationFilename = m
                            .name
                            .clone()
                            .try_into()
                            .expect("Failed to parse file name");
                        db_name == file_name
                    })
                    .expect("Migration file not found in db")
            };

            match &file_name {
                MigrationFilename::Up(up) => {
                    // select * from migration where name = up;
                    // name, timestamp and checksum_up
                    // let mig = Migration::get_by_filename(db_migrations.clone(), up.clone())
                    //     .expect("Migration file not found in db");
                    let found_db_mig = found_db_mig(file_name.clone());
                    assert_eq!(found_db_mig.name, file_name.to_string());
                    assert_eq!(found_db_mig.timestamp, timestamp);
                }
                MigrationFilename::Down(down) => {
                    // select * from migration where name = down.to_up();
                    // name, timestamp and checksum_up
                    let file_name = file_name.to_up();
                    let found_db_mig = found_db_mig(file_name.clone());
                    assert_eq!(found_db_mig.name, file_name.to_string());
                    assert_eq!(found_db_mig.timestamp, timestamp);
                }
                MigrationFilename::Unidirectional(uni) => {
                    // select * from migration where name = down;
                    // name, timestamp and checksum_up
                    let found_db_mig = found_db_mig(file_name.clone());
                    assert_eq!(found_db_mig.name, file_name.to_string());
                    assert_eq!(found_db_mig.timestamp, timestamp);
                }
            };
        }
        // Only up migration filenames are stored in the db since
        // we can always derive the down name from it.

        assert_eq!(basename.to_string(), test_migration_name.to_string());
        assert_eq!(
            file_name.to_string(),
            format!("{timestamp}_{basename}.{extension}"),
            "File name should be in the format of {timestamp}_{basename}.{extension}"
        );
    }
}

fn runtime_config(mode: Mode) -> DatabaseConnection {
    DatabaseConnection::builder()
        .db("test".into())
        .ns("test".into())
        .user("root".into())
        .pass("root".into())
        .mode(mode)
        .prune(false)
        .url(UrlDb::Memory)
        .build()
}

struct AssertionArg {
    db: Surreal<Any>,
    mig_files_count: u8,
    db_mig_count: u8,
    migration_files_dir: PathBuf,
    test_migration_name: &'static str,
}
async fn assert_with_db_instance(args: AssertionArg) {
    let AssertionArg {
        db,
        mig_files_count,
        db_mig_count,
        migration_files_dir,
        test_migration_name,
    } = args;

    let db_migrations = Migration::get_all_desc(db).await;
    let migration_files = read_migs_from_dir(migration_files_dir.clone());

    dbg!(&db_migrations);
    dbg!(&db_mig_count);
    assert_eq!(
        db_migrations.len() as u8,
        db_mig_count,
        "No migrations should be created in the database because we set run to false"
    );
    assert_eq!(
            migration_files.len() as u8,
            mig_files_count,
            "New migration files should not be created on second init. They must be reset instead if you want to change the reversible type."
        );
    assert_migration_files_presence_and_format(migration_files, db_migrations, test_migration_name);
}

struct TestConfig {
    reversible: bool,
    run: bool,
    mig_files_count: u8,
    db_mig_count: u8,
    mode: Mode,
}

async fn test_init(config: TestConfig) {
    let TestConfig {
        reversible,
        run,
        mig_files_count,
        db_mig_count,
        mode,
    } = config;

    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    fs::create_dir_all(temp_test_migration_dir).expect("Failed to create dir");
    let test_migration_name = "test_migration";
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());
    let runtime_config = runtime_config(mode);

    assert_eq!(migration_files.len(), 0);

    let init = Init::builder()
        .name(test_migration_name.to_string())
        .reversible(reversible)
        .run(run)
        .build();

    let mut migrator1 = Migrator::builder()
        .subcmd(SubCommand::Init(init))
        .verbose(3)
        .migrations_dir(temp_test_migration_dir.clone())
        .runtime_config(runtime_config)
        .build();
    let resources = Resources;
    let resources_v2 = ResourcesV2;
    let mock_prompter = MockPrompter { confirmation: true };

    // 1st run
    migrator1
        .run_fn(resources.clone(), mock_prompter.clone())
        .await;
    let cli_db = migrator1.db().clone();

    // let assert_migration_data_static = || async {
    //     assert_with_db_instance(AssertionArg {
    //         db: cli_db.clone(),
    //         mig_files_count,
    //         db_mig_count,
    //         migration_files_dir: temp_test_migration_dir.clone(),
    //         test_migration_name,
    //     })
    //     .await;
    // };

    // First time, should create migration files and db records
    // assert_migration_data_static().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        mig_files_count,
        db_mig_count,
        migration_files_dir: temp_test_migration_dir.clone(),
        test_migration_name,
    })
    .await;

    // Initialize the 2nd time with same codebase resources. Should not allow creation the second time.
    migrator1
        .run_fn(resources.clone(), mock_prompter.clone())
        .await;

    // Second time, should not create migration files nor db records. i.e should be idempotent/
    // Remain the same as the first time.
    // assert_migration_data_static().await;

    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        mig_files_count,
        db_mig_count: 1,
        migration_files_dir: temp_test_migration_dir.clone(),
        test_migration_name,
    })
    .await;

    // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
    migrator1.run_fn(resources_v2, mock_prompter).await;

    // assert_migration_data_static().await;

    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        mig_files_count,
        db_mig_count: 1,
        migration_files_dir: temp_test_migration_dir.clone(),
        test_migration_name,
    })
    .await;
}

#[tokio::test]
async fn test_duplicate_up_only_init_without_run_strict() {
    let reversible = false;
    let run = false;
    let mode = Mode::Strict;
    let mig_files_count = 1;
    let db_mig_count = 1;

    // test_init(TestConfig {
    //     reversible: false,
    //     run: false,
    //     mode: Mode::Strict,
    //     mig_files_count: 1,
    //     db_mig_count: 1,
    // })
    // .await;
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    fs::create_dir_all(temp_test_migration_dir).expect("Failed to create dir");
    let test_migration_name = "test_migration";
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());
    let runtime_config = runtime_config(mode);

    assert_eq!(migration_files.len(), 0);

    let init = Init::builder()
        .name(test_migration_name.to_string())
        .reversible(reversible)
        .run(run)
        .build();

    let mut migrator1 = Migrator::builder()
        .subcmd(SubCommand::Init(init))
        .verbose(3)
        .migrations_dir(temp_test_migration_dir.clone())
        .runtime_config(runtime_config)
        .build();
    let resources = Resources;
    let resources_v2 = ResourcesV2;
    let mock_prompter = MockPrompter { confirmation: true };

    // 1st run
    migrator1
        .run_fn(resources.clone(), mock_prompter.clone())
        .await;
    let cli_db = migrator1.db().clone();

    // let assert_migration_data_static = || async {
    //     assert_with_db_instance(AssertionArg {
    //         db: cli_db.clone(),
    //         mig_files_count,
    //         db_mig_count,
    //         migration_files_dir: temp_test_migration_dir.clone(),
    //         test_migration_name,
    //     })
    //     .await;
    // };

    // First time, should create migration files and db records
    // assert_migration_data_static().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        mig_files_count: 1,
        db_mig_count: 0,
        migration_files_dir: temp_test_migration_dir.clone(),
        test_migration_name,
    })
    .await;

    // Initialize the 2nd time with same codebase resources. Should not allow creation the second time.
    migrator1
        .run_fn(resources.clone(), mock_prompter.clone())
        .await;

    // Second time, should not create migration files nor db records. i.e should be idempotent/
    // Remain the same as the first time.
    // assert_migration_data_static().await;

    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        mig_files_count,
        db_mig_count: 0,
        migration_files_dir: temp_test_migration_dir.clone(),
        test_migration_name,
    })
    .await;

    // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
    migrator1.run_fn(resources_v2, mock_prompter).await;

    // assert_migration_data_static().await;

    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        mig_files_count: 1,
        db_mig_count: 0,
        migration_files_dir: temp_test_migration_dir.clone(),
        test_migration_name,
    })
    .await;
}

// #[tokio::test]
// async fn test_duplicate_up_only_init_and_run_strict() {
//     test_init(TestConfig {
//         reversible: false,
//         run: true,
//         mode: Mode::Strict,
//         mig_files_count: 1,
//         db_mig_count: 1,
//     })
//     .await;
// }
//
// #[tokio::test]
// async fn test_duplicate_bidirectional_up_and_down_init_without_run_strict() {
//     test_init(TestConfig {
//         reversible: true,
//         run: false,
//         mode: Mode::Strict,
//         mig_files_count: 2,
//         db_mig_count: 0,
//     })
//     .await;
// }
//
// #[tokio::test]
// async fn test_duplicate_bidirectional_up_and_down_init_and_run_strict() {
//     test_init(TestConfig {
//         reversible: true,
//         run: true,
//         mode: Mode::Strict,
//         mig_files_count: 2,
//         db_mig_count: 1,
//     })
//     .await;
// }
//
// #[tokio::test]
// async fn test_duplicate_up_only_init_without_run_relaxed() {
//     test_init(TestConfig {
//         reversible: false,
//         run: false,
//         mode: Mode::Lax,
//         mig_files_count: 1,
//         db_mig_count: 0,
//     })
//     .await;
// }
//
// #[tokio::test]
// async fn test_duplicate_up_only_init_and_run_relaxed() {
//     test_init(TestConfig {
//         reversible: false,
//         run: true,
//         mode: Mode::Lax,
//         mig_files_count: 1,
//         db_mig_count: 1,
//     })
//     .await;
// }
//
// #[tokio::test]
// async fn test_duplicate_bidirectional_up_and_down_init_without_run_relaxed() {
//     test_init(TestConfig {
//         reversible: true,
//         run: false,
//         mode: Mode::Lax,
//         mig_files_count: 2,
//         db_mig_count: 0,
//     })
//     .await;
// }
//
// #[tokio::test]
// async fn test_duplicate_bidirectional_up_and_down_init_and_run_relaxed() {
//     test_init(TestConfig {
//         reversible: true,
//         run: true,
//         mode: Mode::Lax,
//         mig_files_count: 2,
//         db_mig_count: 1,
//     })
//     .await;
// }
