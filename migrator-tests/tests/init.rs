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
        migration_cli_fn, Cli, Init, Migration, MockPrompter, SubCommand,
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

#[tokio::test]
async fn test_up_only_init_without_run() {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    fs::create_dir_all(temp_test_migration_dir).expect("Failed to create dir");
    let test_migration_name = "test_migration";
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());
    assert_eq!(migration_files.len(), 0);

    let runtime_config = RuntimeConfig::builder().url(UrlDb::Memory).build();

    let shared_all = SharedAll::builder()
        .migrations_dir(temp_test_migration_dir.into())
        .verbose(4)
        .build();

    let init = Init::builder()
        .name(test_migration_name.to_string())
        .reversible(false) // when false, it's up only or one way or unidirectional
        .run(false)
        .runtime_config(runtime_config)
        .shared_all(shared_all)
        .build();

    dbg!(&init);
    println!("init: {:#?}", init);

    let cli = Cli::new(SubCommand::Init(init));
    let resources = Resources;
    // let resourcesV2 = ResourcesV2;
    let mock_prompter = MockPrompter { confirmation: true };
    let db = migration_cli_fn(cli, resources, mock_prompter).await;

    let migrations = Migration::get_all(db.clone()).await;
    let migration_files = read_migs_from_dir(temp_test_migration_dir.clone());

    assert_eq!(migrations.len(), 1);
    assert_eq!(migration_files.len(), 1);
    // for f in migration_files.iter() {
    //     let binding = f.as_ref().expect("Failed to read dir").path();
    //     let file_name = binding
    //         .file_name() //.expect("Failed to get file namezz
    //         .expect("Failed to get file name")
    //         .to_str()
    //         .expect("Failed to get file name");
    //     let file_name_parsed =
    //         MigrationFilename::try_from(file_name.to_string()).expect("Failed to parse file name");
    //     assert_eq!(
    //         file_name_parsed.timestamp(),
    //         file_name
    //             .split('_')
    //             .next()
    //             .expect("Failed to get timestamp")
    //             .parse::<u64>()
    //             .expect("Failed to parse timestamp")
    //             .into()
    //     );
    //
    //     if file_name.ends_with("_up.surql") {
    //         assert!(file_name.ends_with(&format!("{}_up.surql", test_migration_name)));
    //     } else if file_name.ends_with("_down.surql") {
    //         assert!(file_name.ends_with(&format!("{}_down.surql", test_migration_name)));
    //     }
    // }
}
