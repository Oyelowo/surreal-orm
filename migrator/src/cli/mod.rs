pub mod config;
mod down;
mod generate;
mod init;
mod list;
mod prune;
mod reset;
mod up;

pub use down::{Down, RollbackStrategy};
pub use generate::Generate;
pub use init::Init;
pub use list::{List, Status};
pub use prune::Prune;
pub use reset::Reset;
use surrealdb::{engine::any::Any, Surreal};
pub use up::{Up, UpdateStrategy};

use clap::Parser;
use surreal_query_builder::DbResources;

use crate::{Prompter, RealPrompter};

use self::config::{RuntimeConfig, SetupDb};

/// Surreal ORM CLI
#[derive(Parser, Debug)]
#[clap(name = "SurrealOrm", about = "Surreal ORM CLI")]
pub struct Cli {
    /// Subcommand: generate, up, down, list
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Cli {
    pub fn _new(sub_command: SubCommand) -> Self {
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
    let mut setup = SetupDb::new(RuntimeConfig::default()).await;

    let _ = migration_cli_fn(cli, &mut setup,  codebase_resources, RealPrompter).await;
    ()
}

pub async fn migration_cli_fn(
    cli: Cli,
    setup: &mut SetupDb,
    codebase_resources: impl DbResources,
    prompter: impl Prompter,
) -> Surreal<Any> {

    let db = match cli.subcmd {
        SubCommand::Init(init) => init.run(codebase_resources, prompter, setup).await,
        SubCommand::Generate(generate) => generate.run(codebase_resources, prompter, setup).await,
        SubCommand::Up(up) => up.run(setup).await,
        SubCommand::Down(down) => down.run(setup).await,
        SubCommand::Prune(prune) => prune.run(setup).await,
        SubCommand::List(prune) => prune.run(setup).await,
        SubCommand::Reset(reset) => reset.run(codebase_resources, prompter, setup).await,
    };
    db
}
