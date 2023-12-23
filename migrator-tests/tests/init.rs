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

#[tokio::test]
async fn test_up_only_init_without_run() {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let test_migration_name = "test_migration";
    let _ = std::fs::read_dir(temp_test_migration_dir).expect_err("No such file or directory");

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

    let cli = Cli::new(SubCommand::Init(init));
    let resources = Resources;
    let resourcesV2 = ResourcesV2;
    let mock_prompter = MockPrompter { confirmation: true };
    let db = migration_cli_fn(cli, resources, mock_prompter).await;

    let migrations = select(All)
        .from(Migration::table_name())
        .return_many::<Migration>(db.clone())
        .await
        .unwrap();
    //
    // ()
    todo!()
}
