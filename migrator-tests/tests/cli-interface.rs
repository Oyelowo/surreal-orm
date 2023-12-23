// use std::{
//     io::Write,
//     process::{Command, Stdio},
// };
use surreal_models::migrations::{Resources, ResourcesV2};
// use tempfile::tempdir;
//
use surreal_orm::migrator::{
    config::{RuntimeConfig, SetupDb, SharedAll, UrlDb},
    migration_cli_fn, Cli, FalseMockPrompter, Init, SubCommand, TrueMockPrompter,
};

#[tokio::test]
async fn test_generate_command_success() {
    let mut setup = SetupDb::new(RuntimeConfig::default()).await;
    let runtime_config = RuntimeConfig::builder().url(UrlDb::Memory).build();

    let shared_all = SharedAll::builder()
        .migrations_dir("".into())
        .verbose(4)
        .build();
    let init = Init::builder()
        .name("test migration 1".to_string())
        .reversible(true)
        .run(true)
        .runtime_config(runtime_config)
        .shared_all(shared_all)
        .build();

    let cli = Cli::new(SubCommand::Init(init));
    let resources = Resources;
    let resourcesV2 = ResourcesV2;
    let prompter_returns_true = TrueMockPrompter;
    let prompter_returns_false = FalseMockPrompter;
    let db = migration_cli_fn(cli, resources, TrueMockPrompter).await;
    //
    // let migrations = select(All)
    //     .from(Migration::table_name())
    //     .return_many::<Migration>(db.clone())
    //     .await
    //     .unwrap();
    //
    // ()
    todo!()
}
