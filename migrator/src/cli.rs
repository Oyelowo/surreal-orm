use clap::Parser;
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
    /// Run migrations
    Run(Run),
    /// Rollback migrations
    Rollback(Rollback),
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
struct Run {
    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    shared_run_and_rollback: SharedRunAndRollBack,
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
}

#[derive(Parser, Debug)]
struct SharedRunAndRollBack {
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

    #[clap(short, long, help = "Database name")]
    db: Option<String>,

    #[clap(short, long, help = "Namespace name")]
    ns: Option<String>,

    /// users scope
    #[clap(short, long, help = "Scope")]
    sc: Option<String>,

    #[clap(short, long, help = "User name")]
    user: Option<String>,

    #[clap(short, long, help = "Password")]
    pass: Option<String>,
}

/// Rollback migrations
#[derive(Parser, Debug)]
struct Rollback {
    /// Rollback to the latest migration
    #[clap(long, help = "Rollback to the latest migration")]
    latest: bool,
    /// Rollback by count
    #[clap(long, help = "Rollback by count")]
    by_count: Option<u32>,
    /// Rollback till a specific migration ID
    #[clap(long, help = "Rollback till a specific migration ID")]
    till: Option<String>,

    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    shared_run_and_rollback: SharedRunAndRollBack,
}

use std::fmt::Display;
use std::str::FromStr;

/// Run migration cli
/// # Example
/// ```rust
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
        SubCommand::Run(run) => {
            let db = setup_db(&user_provided_db, &run.shared_run_and_rollback).await;

            if let Some(path) = run.shared_all.migrations_dir {
                files_config = files_config.custom_path(path)
            };
            if run.shared_all.reversible {
                println!("Running two way migrations");
                files_config
                    .two_way()
                    .run_pending_migrations(db.clone())
                    .await
                    .expect("Failed to run migrations");
            } else {
                println!("Running one way migrations");
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
            println!("Database: {:?}", info);
        }
        SubCommand::Rollback(rollback) => {
            let db = setup_db(&user_provided_db, &rollback.shared_run_and_rollback).await;

            if let Some(path) = rollback.shared_all.migrations_dir {
                files_config = files_config.custom_path(path)
            };
            let rollback_strategy = if rollback.latest {
                RollbackStrategy::Latest
            } else if let Some(by_count) = rollback.by_count {
                RollbackStrategy::ByCount(by_count)
            } else if let Some(till) = rollback.till {
                RollbackStrategy::UntilMigrationFileName(till.try_into().unwrap())
            } else {
                RollbackStrategy::Latest
            };

            files_config
                .two_way()
                .rollback_migrations(rollback_strategy, db.clone())
                .await
                .expect("Failed to rollback migrations");
        }
    }
}

async fn setup_db(
    user_provided_db: &Option<Surreal<Any>>,
    shared_run_and_rollback: &SharedRunAndRollBack,
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

async fn init_db(shared: &SharedRunAndRollBack, db: Surreal<Any>) -> Surreal<Any> {
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
            println!("User and password not provided, using root default");
            // db.signin(Root {
            //     username: "root",
            //     password: "root",
            // })
            // .await
            // .expect("Failed to signin");
        }
    };

    if let Some(db_name) = &shared.db {
        println!("Using db {}", db_name);
        db.use_db(db_name).await.expect("Failed to use db");
    }

    if let Some(ns_name) = &shared.ns {
        db.use_ns(ns_name).await.expect("Failed to use ns");
    }
    db
}
