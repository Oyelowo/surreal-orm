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
pub use up::{FastForwardDelta, Up, UpdateStrategy};

use clap::{ArgAction, Parser};
use surreal_query_builder::DbResources;

use self::config::DatabaseConnection;
use crate::{MigrationConfig, MockPrompter, Mode, Prompter, RealPrompter, RenameOrDelete};

/// Surreal ORM CLI
#[derive(Parser, Debug, Clone, TypedBuilder)]
#[clap(name = "SurrealOrm", about = "Surreal ORM CLI")]
#[command(version)]
pub struct Migrator {
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

    #[arg(
        value_enum,
        global = true,
        long,
        help = "If to be strict or lax. Strictness validates the migration files against the database e.g doing checksum checks to make sure.\
            that file contents and valid and also checking filenames. Lax does not.",
        default_value_t = Mode::Strict,
    )]
    pub(crate) mode: Mode,

    #[command(flatten)]
    pub(crate) db_connection: DatabaseConnection,
}

impl Migrator {
    pub async fn setup(&mut self) {
        self.setup_logging();
        self.setup_db().await;
    }

    pub fn db(&self) -> Surreal<Any> {
        self.db_connection.db().expect("Failed to get db")
    }

    pub fn set_cmd(&mut self, cmd: SubCommand) {
        self.subcmd = cmd;
    }

    pub fn set_db_connection_from_migrator(&mut self, migrator: &Migrator) {
        self.db_connection = migrator.db_connection.clone();
    }

    pub fn set_db(&mut self, db: Surreal<Any>) {
        self.db_connection.db_connection = Some(db);
    }

    pub fn file_manager(&self) -> MigrationConfig {
        let fm_init = MigrationConfig::builder()
            .custom_path(self.migrations_dir.clone())
            .mode(self.mode);

        // let fm = fm_init.build().detect_migration_type().ok();

        // fm_init
        //     .migration_flag(fm_init.build().detect_migration_type().ok())
        //     .build()
        fm_init.build()
    }

    /// Run migration cli
    /// # Example
    /// ```rust, ignore
    /// use surreal_models::migrations::Resources;
    /// use surreal_orm::migrator::{self, MigrationConfig, RollbackStrategy};
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
    ///    // include example usage as rust doc
    ///     Cli::run(Resources).await;
    ///
    ///
    ///    // Interact programmatically
    ///    
    /// }
    /// ```
    pub async fn run(codebase_resources: impl DbResources) {
        let mut cli = Self::parse();
        cli.setup_logging();
        cli.run_fn(codebase_resources, RealPrompter).await;
    }

    pub async fn run_test(codebase_resources: impl DbResources) {
        let mut cli = Self::parse();
        cli.setup_logging();
        cli.run_fn(
            codebase_resources,
            MockPrompter::builder()
                .allow_empty_migrations_gen(false)
                .rename_or_delete_single_field_change(RenameOrDelete::Rename)
                .build(),
        )
        .await;
    }

    pub async fn run_fn(&mut self, codebase_resources: impl DbResources, prompter: impl Prompter) {
        self.setup_db().await;

        match self.subcmd.clone() {
            SubCommand::Init(init) => init.run(self, codebase_resources, prompter).await,
            SubCommand::Generate(generate) => {
                generate.run(self, codebase_resources, prompter).await
            }
            SubCommand::Up(up) => up.run(self).await,
            SubCommand::Down(down) => down.run(self).await,
            SubCommand::Prune(prune) => prune.run(self).await,
            SubCommand::List(prune) => prune.run(self).await,
            SubCommand::Reset(reset) => reset.run(self, codebase_resources, prompter).await,
        };
    }

    pub async fn run_up_fn(&mut self) {
        self.setup_db().await;

        match self.subcmd.clone() {
            SubCommand::Up(up) => up.run(self).await,
            _ => panic!("Expected up subcommand"),
        };
    }

    pub(crate) fn setup_logging(&self) {
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

    pub async fn setup_db(&mut self) {
        if let None = self.db_connection.db_connection {
            self.db_connection.setup().await;
        }
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
