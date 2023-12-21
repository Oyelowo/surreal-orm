use std::{
    io::Write,
    process::{Command, Stdio},
};
use surreal_models::migrations::{Resources, ResourcesV2};
use tempfile::tempdir;

use surreal_orm::migrator::{migration_cli_fn, Cli, TrueMockPrompter};

#[tokio::test]
async fn test_generate_command_success() {
    let cli = Cli::_new(todo!());
    let resources = Resources;
    let resourcesV2 = ResourcesV2;
    migration_cli_fn(cli, resources, TrueMockPrompter).await;
}
