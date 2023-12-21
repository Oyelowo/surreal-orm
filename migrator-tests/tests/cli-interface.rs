use std::{
    io::Write,
    process::{Command, Stdio},
};
use surreal_models::migrations::{Resources, ResourcesV2};
use tempfile::tempdir;

use surreal_orm::{
    migrator::{
        config::{RuntimeConfig, SetupDb},
        migration_cli_fn, Cli, FalseMockPrompter, Migration, TrueMockPrompter,
    },
    statements::select,
    All, ReturnableSelect,
};

#[tokio::test]
async fn test_generate_command_success() {
    let mut setup = SetupDb::new(RuntimeConfig::default()).await;
    let cli = Cli::_new(todo!());
    let resources = Resources;
    let resourcesV2 = ResourcesV2;
    let prompter_returns_true = TrueMockPrompter;
    let prompter_returns_false = FalseMockPrompter;
    let db = migration_cli_fn(&mut setup, cli, resources, TrueMockPrompter).await;

    let migrations = select(All)
        .from(Migration::table_name())
        .return_many::<Migration>(db.clone())
        .await
        .unwrap();

    ()
}
