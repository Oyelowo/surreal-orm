mod arg_parser;
pub mod config;
mod down;
mod generate;
mod init;
mod list;
mod prune;
mod reset;
mod shared_traits;
mod up;

use std::path::PathBuf;

pub use arg_parser::*;
pub use down::{Down, RollbackStrategy};
pub use generate::Generate;
pub use init::Init;
pub use list::{List, Status};
pub use prune::Prune;
pub use reset::Reset;
pub use shared_traits::DbConnection;

use surrealdb::{engine::any::Any, Surreal};
use typed_builder::TypedBuilder;
pub use up::{Up, UpdateStrategy};

use clap::{ArgAction, Args, Parser};
use surreal_query_builder::DbResources;

use crate::{MigrationConfig, Mode, Prompter, RealPrompter};

use self::config::{RuntimeConfig, UrlDb};

/// Surreal ORM CLI
#[derive(Parser, Debug, Clone, TypedBuilder)]
#[clap(name = "SurrealOrm", about = "Surreal ORM CLI")]
#[command(version)]
pub struct Cli {
    /// Subcommand: generate, up, down, list
    #[command(subcommand)]
    subcmd: SubCommand,

    /// Optional custom migrations dir
    #[arg(global = true, short, long, help = "Optional custom migrations dir")]
    #[builder(default, setter(strip_option))]
    pub migrations_dir: Option<PathBuf>,

    /// Sets the level of verbosity e.g -v, -vv, -vvv, -vvvv
    #[arg(global = true, short, long, action = ArgAction::Count, default_value_t=3)]
    pub(crate) verbose: u8,

    #[command(flatten)]
    pub(crate) runtime_config: RuntimeConfig,
}

impl Cli {
    // pub fn new(sub_command: SubCommand) -> Self {
    //     Self {
    //         subcmd: sub_command,
    //         runtime_config: todo!(),
    //         db: None,
    //     }
    // }

    pub fn setup_logging(&self) {
        let verbosity = self.verbose;
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

    pub fn setup_db(&mut self) {
        self.runtime_config.setup();
    }

    pub fn setup(&mut self) {
        self.setup_db();
        self.setup_logging();
    }

    pub fn db(&self) -> Surreal<Any> {
        self.runtime_config.db().expect("Failed to get db")
    }

    pub fn file_manager(&self) -> MigrationConfig {
        let fm_init = MigrationConfig::builder()
            .custom_path(self.migrations_dir.clone())
            .mode(self.runtime_config.mode);

        // let fm = fm_init.build().detect_migration_type().ok();

        // fm_init
        //     .migration_flag(fm_init.build().detect_migration_type().ok())
        //     .build()
        fm_init.build()
    }
}

/// Subcommands
#[derive(Parser, Debug, Clone)]
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
    let mut cli = Cli::parse();

    let _ = migration_cli_fn(&mut cli, codebase_resources, RealPrompter).await;
}

pub async fn migration_cli_fn(
    cli: &mut Cli,
    codebase_resources: impl DbResources,
    prompter: impl Prompter,
) -> Surreal<Any> {
    match cli.subcmd.clone() {
        SubCommand::Init(init) => init.run(cli, codebase_resources, prompter).await,
        SubCommand::Generate(generate) => generate.run(cli, codebase_resources, prompter).await,
        SubCommand::Up(up) => up.run(cli).await,
        SubCommand::Down(down) => down.run(cli).await,
        SubCommand::Prune(prune) => prune.run(cli).await,
        SubCommand::List(prune) => prune.run(cli).await,
        SubCommand::Reset(reset) => reset.run(cli, codebase_resources, prompter).await,
    };

    cli.db().clone()
}
