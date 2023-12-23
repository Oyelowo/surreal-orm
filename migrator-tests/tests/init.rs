use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use surrealdb::engine::any::Any;
use surrealdb::Surreal;
// use std::{
//     io::Write,
//     process::{Command, Stdio},
// };
use surreal_models::migrations::{Resources, ResourcesV2};
use surreal_orm::{
    migrator::{
        config::{RuntimeConfig,  SharedAll, UrlDb},
        migration_cli_fn, Cli, Init, Migration, MigrationFilename, MockPrompter, Mode, SubCommand,
    },
};
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
            .file_name() //.expect("Failed to get file namezz
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

#[tokio::test]
async fn test_duplicate_up_only_init_without_run() {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    fs::create_dir_all(temp_test_migration_dir).expect("Failed to create dir");
    let test_migration_name = "test_migration";
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());
    assert_eq!(migration_files.len(), 0);

    let runtime_config = RuntimeConfig::builder()
        .db("test".into())
        .ns("test".into())
        .user("root".into())
        .pass("root".into())
        .mode(Mode::Strict)
        .prune(false)
        .url(UrlDb::Memory)
        .build();

    let shared_all = SharedAll::builder()
        .migrations_dir(temp_test_migration_dir.into())
        .verbose(3)
        .build();

    let init = Init::builder()
        .name(test_migration_name.to_string())
        .reversible(false) // when false, it's up only or one way or unidirectional
        // We are setting run to false here
        // This means that we are not running the migrations after generation
        // This is the default behavior
        // Which means new migration metadata will not be created in
        // the database, nor would the generated migration files be run
        .run(false)
        .runtime_config(runtime_config)
        .shared_all(shared_all)
        .build();

    let cli = Cli::new(SubCommand::Init(init));
    let resources = Resources;
    let resourcesV2 = ResourcesV2;
    let mock_prompter = MockPrompter { confirmation: true };
    let db = migration_cli_fn(cli.clone(), resources.clone(), mock_prompter.clone()).await;
    
    let migrations = Migration::get_all(db.clone()).await;
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());

    assert_eq!(
        migrations.len(),
        0,
        "No migrations should be created in the database because we set run to false"
    );
    assert_eq!(
        migration_files.len(),
        1,
        "One migration file should be created"
    );

    assert_migration_files_presence_and_format(migration_files, test_migration_name);

    // Initialize the 2nd time with same codebase resources. Should not allow creation the second time.
    let db = migration_cli_fn(cli.clone(), resources.clone(), mock_prompter.clone()).await;

    let migrations = Migration::get_all(db.clone()).await;
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());

    assert_eq!(
        migrations.len(),
        0,
        "No migrations should be created in the database because we set run to false"
    );
    assert_eq!(
        migration_files.len(),
        1,
        "New migration files should not be created on second init. They must be reset instead if you want to change the reversible type."
    );
    assert_migration_files_presence_and_format(migration_files, test_migration_name);


    // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
    let db = migration_cli_fn(cli, resourcesV2, mock_prompter).await;

    let migrations = Migration::get_all(db.clone()).await;
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());

    assert_eq!(
        migrations.len(),
        0,
        "No migrations should be created in the database because we set run to false"
    );
    assert_eq!(
        migration_files.len(),
        1,
        "New migration files should not be created on second init. They must be reset instead if you want to change the reversible type."
    );
    assert_migration_files_presence_and_format(migration_files, test_migration_name);

}


#[tokio::test]
async fn test_duplicate_up_only_init_and_run() {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    fs::create_dir_all(temp_test_migration_dir).expect("Failed to create dir");
    let test_migration_name = "test_migration";
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());
    assert_eq!(migration_files.len(), 0);

    let runtime_config = RuntimeConfig::builder()
        .db("test".into())
        .ns("test".into())
        .user("root".into())
        .pass("root".into())
        .mode(Mode::Strict)
        .prune(false)
        .url(UrlDb::Memory)
        .build();

    let shared_all = SharedAll::builder()
        .migrations_dir(temp_test_migration_dir.into())
        .verbose(3)
        .build();

    let init = Init::builder()
        .name(test_migration_name.to_string())
        .reversible(false) // when false, it's up only or one way or unidirectional
        // We are setting run to true here to run the newly
        // generated migration files against the specified database instance.
        .run(true)
        .runtime_config(runtime_config)
        .shared_all(shared_all)
        .build();

    let cli = Cli::new(SubCommand::Init(init));
    let resources = Resources;
    let resourcesV2 = ResourcesV2;
    let mock_prompter = MockPrompter { confirmation: true };
    
    let assert_with_db_instance = |db: Surreal<Any>, mig_files_count: u8, db_mig_count: u8| async move {
        let migrations = Migration::get_all(db).await;
        let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());

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
    };


    // 1st run
    let db = migration_cli_fn(cli.clone(), resources.clone(), mock_prompter.clone()).await;

    assert_with_db_instance(db.clone(), 1, 1);

    // Initialize the 2nd time with same codebase resources. Should not allow creation the second time.
    let db2 = migration_cli_fn(cli.clone(), resources.clone(), mock_prompter.clone()).await;

    assert_with_db_instance(db.clone(), 1, 1);
    assert_with_db_instance(db2.clone(), 1, 0);


    // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
    let db3 = migration_cli_fn(cli, resourcesV2, mock_prompter).await;

    assert_with_db_instance(db.clone(), 1, 1);
    assert_with_db_instance(db3.clone(), 1, 0);

}

