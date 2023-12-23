pub mod config;
mod down;
mod generate;
mod init;
mod list;
mod prune;
mod reset;
mod shared_traits;
mod up;

use async_trait::async_trait;
pub use down::{Down, RollbackStrategy};
pub use generate::Generate;
pub use init::Init;
pub use list::{List, Status};
pub use prune::Prune;
pub use reset::Reset;
pub use shared_traits::DbConnection;

use surrealdb::{engine::any::Any, Surreal};
pub use up::{Up, UpdateStrategy};

use clap::Parser;
use surreal_query_builder::DbResources;

use crate::{Prompter, RealPrompter};

/// Surreal ORM CLI
#[derive(Parser, Debug)]
#[clap(name = "SurrealOrm", about = "Surreal ORM CLI")]
pub struct Cli {
    /// Subcommand: generate, up, down, list
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Cli {
    pub fn new(sub_command: SubCommand) -> Self {
        Self {
            subcmd: sub_command,
        }
    }
}

/// Subcommands
#[derive(Parser, Debug)]
pub enum SubCommand {
    /// Init migrations
    Init(Init),
    /// Generate migrations
    #[clap(alias = "gen")]
    Generate(Generate),
    /// Run migrations forward
    Up(Up),
    /// Rollback migrations
    Down(Down),
    /// Reset migrations. Deletes all migration files, migration table and reinitializes
    /// migrations
    Reset(Reset),
    /// List migrations
    #[clap(alias = "ls")]
    List(List),
    /// Delete Unapplied local migration files that have not been applied to the current database instance
    Prune(Prune),
}

#[async_trait]
impl crate::DbConnection for SubCommand {
    async fn create_and_set_connection(&mut self) {
        match self {
            SubCommand::Init(init) => init.create_and_set_connection().await,
            SubCommand::Generate(generate) => generate.create_and_set_connection().await,
            SubCommand::Up(up) => up.create_and_set_connection().await,
            SubCommand::Down(down) => down.create_and_set_connection().await,
            SubCommand::List(list) => list.create_and_set_connection().await,
            SubCommand::Prune(prune) => prune.create_and_set_connection().await,
            SubCommand::Reset(reset) => reset.create_and_set_connection().await,
        };
    }

    async fn db(&self) -> Surreal<Any> {
        match self {
            SubCommand::Init(init) => init.db().await,
            SubCommand::Generate(generate) => generate.db().await,
            SubCommand::Up(up) => up.db().await,
            SubCommand::Down(down) => down.db().await,
            SubCommand::List(list) => list.db().await,
            SubCommand::Prune(prune) => prune.db().await,
            SubCommand::Reset(reset) => reset.db().await,
        }
    }
}

impl SubCommand {
    pub fn get_verbosity(&self) -> u8 {
        match self {
            SubCommand::Init(generate) => generate.shared_all.verbose,
            SubCommand::Generate(generate) => generate.shared_all.verbose,
            SubCommand::Up(run) => run.shared_all.verbose,
            SubCommand::Down(rollback) => rollback.shared_all.verbose,
            SubCommand::List(list) => list.shared_all.verbose,
            SubCommand::Prune(prune) => prune.shared_all.verbose,
            SubCommand::Reset(reset) => reset.shared_all.verbose,
        }
    }

    pub fn setup_logging(&self) {
        let verbosity = self.get_verbosity();
        let log_level = match verbosity {
            0 => "error",
            1 => "warn",
            2 => "info",
            3 => "debug",
            _ => "trace",
        };

        std::env::set_var("RUST_LOG", log_level);
        pretty_env_logger::init();
    }
}

/// Run migration cli
/// # Example
/// ```rust, ignore
/// use surreal_models::migrations::Resources;
/// use surreal_orm::migrator::{cli, MigrationConfig, RollbackStrategy};
/// use surrealdb::engine::remote::ws::Ws;
/// use surrealdb::opt::auth::Root;
/// use surrealdb::{Connection, Surreal};
///
/// async fn initialize_db() -> Surreal<surrealdb::engine::remote::ws::Client> {
///     let db = Surreal::new::<Ws>("localhost:8000")
///    .await
///    .expect("Failed to connect to db");
///     
///     db.signin(Root {
///         username: "root",
///         password: "root",
///     })
///     .await
///     .expect("Failed to signin");
///
///     db.use_ns("test").use_db("test").await.unwrap();
///     db
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let db = initialize_db().await;
///    // include example usage as rust doc
///     cli::migration_cli(db, Resources).await;
/// }
/// ```
pub async fn migration_cli(codebase_resources: impl DbResources) {
    let cli = Cli::parse();
    cli.subcmd.setup_logging();

    let _ = migration_cli_fn(cli, codebase_resources, RealPrompter).await;
    ()
}

pub async fn migration_cli_fn(
    mut cli: Cli,
    codebase_resources: impl DbResources,
    prompter: impl Prompter,
) -> Surreal<Any> {
    cli.subcmd.create_and_set_connection().await;

    match &cli.subcmd {
        SubCommand::Init(init) => init.run(codebase_resources, prompter).await,
        SubCommand::Generate(generate) => generate.run(codebase_resources, prompter).await,
        SubCommand::Up(up) => up.run().await,
        SubCommand::Down(down) => down.run().await,
        SubCommand::Prune(prune) => prune.run().await,
        SubCommand::List(prune) => prune.run().await,
        SubCommand::Reset(reset) => reset.run(codebase_resources, prompter).await,
    };

    cli.subcmd.db().await
}
