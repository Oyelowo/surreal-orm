use clap::{ArgAction, Parser};
use surreal_query_builder::statements::info_for;
use surreal_query_builder::{DbResources, Runnable};
use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::{DbInfo, MigrationConfig, RollbackStrategy};

/// Surreal ORM CLI
#[derive(Parser, Debug)]
#[clap(name = "SurrealOrm", about = "Surreal ORM CLI")]
struct Cli {
    /// Subcommand: generate, run, rollback
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Subcommands
#[derive(Parser, Debug)]
enum SubCommand {
    /// Generate migrations
    Generate(Generate),
    /// Run migrations forward
    Up(Up),
    /// Rollback migrations
    Down(Down),
    /// List migrations
    List(List),
}

impl SubCommand {
    pub fn get_verbosity(&self) -> u8 {
        match self {
            SubCommand::Generate(generate) => generate.shared_all.verbose,
            SubCommand::Up(run) => run.shared_all.verbose,
            SubCommand::Down(rollback) => rollback.shared_all.verbose,
            SubCommand::List(list) => list.shared_all.verbose,
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

/// Generate migrations
#[derive(Parser, Debug)]
struct Generate {
    /// Name of the migration
    #[clap(long, help = "Name of the migration")]
    name: String,
    #[clap(flatten)]
    shared_all: SharedAll,
}

/// Run migrations
#[derive(Parser, Debug)]
struct Up {
    /// Run forward to the latest migration
    #[clap(long, help = "Run forward to the next migration")]
    latest: bool,
    /// Run forward by count/number
    #[clap(long, help = "Run forward by the number specified")]
    number: Option<u32>,
    /// Run forward till a specific migration ID
    #[clap(long, help = "Run forward till a specific migration ID")]
    till: Option<String>,
    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    shared_run_and_rollback: RuntimeConfig,
}

#[derive(Clone, Debug, Default)]
pub enum Status {
    #[default]
    Applied,
    Pending,
    All,
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        if s == "pending" {
            Ok(Status::Pending)
        } else if s == "applied" {
            Ok(Status::Applied)
        } else if s == "all" {
            Ok(Status::All)
        } else {
            Err("Invalid status".to_string())
        }
    }
}

/// Run migrations
#[derive(Parser, Debug)]
struct List {
    /// Status of migrations to list
    #[clap(
        long,
        help = "Status of migrations to list. Can be 'applied', 'pending' or 'all'",
        default_value = "applied"
    )]
    status: Option<Status>,
    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    runtime_config: RuntimeConfig,
}

#[derive(Clone, Debug)]
enum Path {
    Memory,
    Others(String),
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Memory => write!(f, "mem://"),
            Path::Others(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for Path {
    type Err = String;

    // Can be one of memory, file:<path>, tikv:<addr>, file://<path>, tikv://<addr>
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        if s == "memory" {
            Ok(Path::Memory)
        } else {
            Ok(Path::Others(s))
            // Err("Invalid path".to_string())
        }
    }
}

#[derive(Parser, Debug)]
struct SharedAll {
    /// Two way migration
    #[clap(short, long, help = "Two-way up & down migration")]
    reversible: bool,

    /// Optional custom migrations dir
    #[clap(short, long, help = "Optional custom migrations dir")]
    migrations_dir: Option<String>,

    /// Sets the level of verbosity e.g -v, -vv, -vvv, -vvvv
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

#[derive(Parser, Debug)]
struct RuntimeConfig {
    /// URL to connect to a database instance.
    ///
    /// Examples:
    /// - Local WebSocket: `ws://localhost:8000`
    /// - Remote WebSocket: `wss://cloud.surrealdb.com`
    /// - HTTP: `http://localhost:8000`
    /// - HTTPS: `https://cloud.surrealdb.com`
    /// - In-Memory: `mem://`
    /// - File-Backed: `file://temp.db`
    /// - IndxDB-Backed: `indxdb://MyDatabase`
    /// - TiKV-Backed: `tikv://localhost:2379`
    /// - FoundationDB-Backed: `fdb://fdb.cluster`
    #[clap(
        long,
        // short,
        value_name = "URL",
        // required = true,
        // about = "URL or path to connect to a database. Supports various backends.",
        help = "Example:\n\
                - ws://localhost:8000\n\
                - wss://cloud.surrealdb.com\n\
                - http://localhost:8000\n\
                - https://cloud.surrealdb.com\n\
                - mem://\n\
                - file://temp.db\n\
                - indxdb://MyDatabase\n\
                - tikv://localhost:2379\n\
                - fdb://fdb.cluster"
    )]
    path: Option<Path>,

    #[clap(long, help = "Database name")]
    db: Option<String>,

    #[clap(long, help = "Namespace name")]
    ns: Option<String>,

    /// users scope
    #[clap(long, help = "Scope")]
    sc: Option<String>,

    #[clap(short, long, help = "User name")]
    user: Option<String>,

    #[clap(short, long, help = "Password")]
    pass: Option<String>,
}

/// Rollback migrations
#[derive(Parser, Debug)]
struct Down {
    /// Rollback to the latest migration
    #[clap(long, help = "Rollback to the previous migration")]
    previous: bool,
    /// Rollback by count/number
    #[clap(long, help = "Rollback by count")]
    number: Option<u32>,
    /// Rollback till a specific migration ID
    #[clap(long, help = "Rollback till a specific migration ID")]
    till: Option<String>,

    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    shared_run_and_rollback: RuntimeConfig,
}

use std::fmt::Display;
use std::str::FromStr;

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
///    let db = Surreal::new::<Ws>("localhost:8000")
///    .await
///    .expect("Failed to connect to db");
///    db.signin(Root {
///         username: "root",
///         password: "root",
/// })
///    .await
///    .expect("Failed to signin");
///    db.use_ns("test").use_db("test").await.unwrap();
///    db
/// }
///    #[tokio::main]
/// async fn main() {
///         let db = initialize_db().await;
///    // include example usage as rust doc
///         cli::migration_cli(db, Resources).await;
/// }
/// ```
pub async fn migration_cli(
    codebase_resources: impl DbResources,
    user_provided_db: Option<Surreal<Any>>,
) {
    let cli = Cli::parse();
    cli.subcmd.setup_logging();

    let mut files_config = MigrationConfig::new().make_strict();
    match cli.subcmd {
        SubCommand::Generate(generate) => {
            let migration_name = generate.name;
            if let Some(path) = generate.shared_all.migrations_dir {
                files_config = files_config.custom_path(path)
            };

            if generate.shared_all.reversible {
                files_config
                    .two_way()
                    .generate_migrations(migration_name, codebase_resources)
                    .await
                    .expect("Failed to generate migrations");
            } else {
                files_config
                    .one_way()
                    .generate_migrations(migration_name, codebase_resources)
                    .await
                    .expect("Failed to generate migrations");
            };
        }
        SubCommand::Up(run) => {
            let db = setup_db(&user_provided_db, &run.shared_run_and_rollback).await;

            if let Some(path) = run.shared_all.migrations_dir {
                files_config = files_config.custom_path(path)
            };
            if run.shared_all.reversible {
                log::info!("Running two way migrations");
                files_config
                    .two_way()
                    .run_pending_migrations(db.clone())
                    .await
                    .expect("Failed to run migrations");
            } else {
                log::info!("Running one way migrations");
                files_config
                    .one_way()
                    .run_pending_migrations(db.clone())
                    .await
                    .expect("Failed to run migrations");
            }
            // define_table("nawa").run(db.clone()).await.unwrap();
            let info = info_for()
                .database()
                .get_data::<DbInfo>(db.clone())
                .await
                .expect("Failed to get db info");
            log::info!("Database: {:?}", info);
        }
        SubCommand::Down(rollback) => {
            let db = setup_db(&user_provided_db, &rollback.shared_run_and_rollback).await;

            if let Some(path) = rollback.shared_all.migrations_dir {
                files_config = files_config.custom_path(path)
            };
            let rollback_strategy = if rollback.previous {
                RollbackStrategy::Previous
            } else if let Some(by_count) = rollback.number {
                RollbackStrategy::Number(by_count)
            } else if let Some(till) = rollback.till {
                RollbackStrategy::Till(till.try_into().unwrap())
            } else {
                RollbackStrategy::Previous
            };

            files_config
                .two_way()
                .rollback_migrations(rollback_strategy, db.clone())
                .await
                .expect("Failed to rollback migrations");
        }
        SubCommand::List(options) => {
            let db = setup_db(&user_provided_db, &options.runtime_config).await;

            if let Some(path) = options.shared_all.migrations_dir {
                files_config = files_config.custom_path(path)
            };
            if options.shared_all.reversible {
                log::info!("Listing two way migrations");
                let migrations = files_config
                    .two_way()
                    .list_migrations(db.clone(), options.status.unwrap_or_default())
                    .await
                    .expect("Failed to get migrations");
                log::info!("Migrations: {:?}", migrations);
            } else {
                log::info!("Listing one way migrations");
                let migrations = files_config
                    .one_way()
                    .list_migrations(db.clone(), options.status.unwrap_or_default())
                    .await
                    .expect("Failed to get migrations");
                log::info!("Migrations: {:?}", migrations);
            }
        }
    }
}

async fn setup_db(
    user_provided_db: &Option<Surreal<Any>>,
    shared_run_and_rollback: &RuntimeConfig,
) -> Surreal<Any> {
    let db_url = shared_run_and_rollback.path.clone();

    match (user_provided_db, &db_url) {
        (Some(user_db), None) => {
            let db = user_db;
            init_db(shared_run_and_rollback, db.clone()).await
        }
        (_, Some(cli_db_url)) => {
            let db = connect(&cli_db_url.to_string()).await.unwrap();
            init_db(shared_run_and_rollback, db.clone()).await
        }
        (None, None) => panic!("No db provided"),
    }
}

async fn init_db(shared: &RuntimeConfig, db: Surreal<Any>) -> Surreal<Any> {
    match (&shared.user, &shared.pass) {
        (Some(u), Some(p)) => {
            db.signin(Root {
                username: u.as_str(),
                password: p.as_str(),
            })
            .await
            .expect("Failed to signin");
        }
        (Some(_), None) => panic!("Password not provided"),
        (None, Some(_)) => panic!("User not provided"),
        _ => {
            log::warn!("User and password not provided, using root default");
            // db.signin(Root {
            //     username: "root",
            //     password: "root",
            // })
            // .await
            // .expect("Failed to signin");
        }
    };

    if let Some(db_name) = &shared.db {
        log::info!("Using db {}", db_name);
        db.use_db(db_name).await.expect("Failed to use db");
    }

    if let Some(ns_name) = &shared.ns {
        db.use_ns(ns_name).await.expect("Failed to use ns");
    }
    db
}
