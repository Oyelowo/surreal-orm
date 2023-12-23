use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

// use std::{
//     io::Write,
//     process::{Command, Stdio},
// };
use surreal_models::migrations::{Resources, ResourcesV2};
// use tempfile::tempdir;
//
use surreal_orm::{
    migrator::{
        config::{RuntimeConfig, SetupDb, SharedAll, UrlDb},
        migration_cli_fn, Cli, Init, Migration, MigrationFilename, MockPrompter, Mode, SubCommand,
    },
    statements::select,
    All, ReturnableSelect,
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
async fn test_up_only_init_without_run() {
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
    // let resourcesV2 = ResourcesV2;
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

    // Initialize the 2nd time
    let db = migration_cli_fn(cli, resources, mock_prompter).await;

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
}
