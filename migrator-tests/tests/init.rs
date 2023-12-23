use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use surreal_models::migrations::{Resources, ResourcesV2};
use surreal_orm::migrator::{
    config::{RuntimeConfig, SharedAll, UrlDb},
    migration_cli_fn, Cli, Init, Migration, MigrationFilename, MockPrompter, Mode, SubCommand,
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
    test_migration_name: &str,
) {
    for f in migration_files.iter() {
        let filepath = f.path();
        let file_name = filepath
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to get file name");
        let file_name_parsed =
            MigrationFilename::try_from(file_name.to_string()).expect("Failed to parse file name");

        let timestamp = file_name_parsed.timestamp();
        let basename = file_name_parsed.basename();
        let extension = file_name_parsed.extension();

        assert_eq!(basename.to_string(), test_migration_name.to_string());
        assert_eq!(
            file_name.to_string(),
            format!("{timestamp}_{basename}.{extension}"),
            "File name should be in the format of {timestamp}_{basename}.{extension}"
        );
    }
}

fn runtime_config() -> RuntimeConfig {
    RuntimeConfig::builder()
        .db("test".into())
        .ns("test".into())
        .user("root".into())
        .pass("root".into())
        .mode(Mode::Strict)
        .prune(false)
        .url(UrlDb::Memory)
        .build()
}

fn shared_all(migrations_dir: PathBuf) -> SharedAll {
    SharedAll::builder()
        .migrations_dir(migrations_dir.into())
        .verbose(3)
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

    let migrations = Migration::get_all(db).await;
    let migration_files = read_migs_from_dir(migration_files_dir.clone());

    assert_eq!(
        migrations.len() as u8,
        db_mig_count,
        "No migrations should be created in the database because we set run to false"
    );
    assert_eq!(
            migration_files.len() as u8,
            mig_files_count,
            "New migration files should not be created on second init. They must be reset instead if you want to change the reversible type."
        );
    assert_migration_files_presence_and_format(migration_files, test_migration_name);
}

struct TestConfig {
    reversible: bool,
    run: bool,
    mig_files_count: u8,
    db_mig_count: u8,
}

async fn test_init(config: TestConfig) {
    let TestConfig {
        reversible,
        run,
        mig_files_count,
        db_mig_count,
    } = config;

    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    fs::create_dir_all(temp_test_migration_dir).expect("Failed to create dir");
    let test_migration_name = "test_migration";
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());
    let runtime_config = runtime_config();
    let shared_all = shared_all(temp_test_migration_dir.clone());

    assert_eq!(migration_files.len(), 0);

    let init = Init::builder()
        .name(test_migration_name.to_string())
        .reversible(reversible)
        .run(run)
        .runtime_config(runtime_config)
        .shared_all(shared_all)
        .build();

    let cli = Cli::new(SubCommand::Init(init));
    let resources = Resources;
    let resources_v2 = ResourcesV2;
    let mock_prompter = MockPrompter { confirmation: true };

    // 1st run
    let db = migration_cli_fn(cli.clone(), resources.clone(), mock_prompter.clone()).await;
    let assert_with_db_instance1 = |db: Surreal<Any>| async move {
        assert_with_db_instance(AssertionArg {
            db: db.clone(),
            mig_files_count,
            db_mig_count,
            migration_files_dir: temp_test_migration_dir.clone(),
            test_migration_name,
        })
        .await;
    };

    assert_with_db_instance1(db.clone()).await;

    // Initialize the 2nd time with same codebase resources. Should not allow creation the second time.
    let db2 = migration_cli_fn(cli.clone(), resources.clone(), mock_prompter.clone()).await;

    assert_with_db_instance1(db.clone()).await;

    assert_with_db_instance(AssertionArg {
        db: db2.clone(),
        mig_files_count,
        db_mig_count: 0,
        migration_files_dir: temp_test_migration_dir.clone(),
        test_migration_name,
    })
    .await;

    // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
    let db3 = migration_cli_fn(cli, resources_v2, mock_prompter).await;

    assert_with_db_instance1(db.clone()).await;

    assert_with_db_instance(AssertionArg {
        db: db3.clone(),
        mig_files_count,
        db_mig_count: 0,
        migration_files_dir: temp_test_migration_dir.clone(),
        test_migration_name,
    })
    .await;
}

#[tokio::test]
async fn test_duplicate_up_only_init_without_run() {
    test_init(TestConfig {
        reversible: false,
        run: false,
        mig_files_count: 1,
        db_mig_count: 0,
    })
    .await;
}

#[tokio::test]
async fn test_duplicate_up_only_init_and_run() {
    test_init(TestConfig {
        reversible: false,
        run: true,
        mig_files_count: 1,
        db_mig_count: 1,
    })
    .await;
}

#[tokio::test]
async fn test_duplicate_bidirectional_up_and_down_init_without_run() {
    test_init(TestConfig {
        reversible: true,
        run: false,
        mig_files_count: 2,
        db_mig_count: 0,
    })
    .await;
}

#[tokio::test]
async fn test_duplicate_bidirectional_up_and_down_init_and_run() {
    test_init(TestConfig {
        reversible: true,
        run: true,
        mig_files_count: 2,
        db_mig_count: 1,
    })
    .await;
}
