use clap::Parser;
use surreal_query_builder::DbResources;
use surrealdb::engine::any::{connect, Any};
use surrealdb::engine::remote::ws::{self, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Connection, Surreal};

use crate::{MigrationConfig, RollbackStrategy};

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
    #[clap(long, default_value = "migration_name_example")]
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

// type X = RocksDb;
// type X = surrealdb::engine::local::;
/// // Connect to a local endpoint
/// let db = connect("ws://localhost:8000").await?;
///
/// // Connect to a remote endpoint
/// let db = connect("wss://cloud.surrealdb.com").await?;
///
/// // Connect using HTTP
/// let db = connect("http://localhost:8000").await?;
///
/// // Connect using HTTPS
/// let db = connect("https://cloud.surrealdb.com").await?;
///
/// // Instantiate an in-memory instance
/// let db = connect("mem://").await?;
///
/// // Instantiate an file-backed instance
/// let db = connect("file://temp.db").await?;
///
/// // Instantiate an IndxDB-backed instance
/// let db = connect("indxdb://MyDatabase").await?;
///
/// // Instantiate a TiKV-backed instance
/// let db = connect("tikv://localhost:2379").await?;
///
/// // Instantiate a FoundationDB-backed instance
/// let db = connect("fdb://fdb.cluster").await?;
/// # Ok(())
/// #
impl FromStr for Path {
    type Err = String;

    // Can be one of memory, file:<path>, tikv:<addr>, file://<path>, tikv://<addr>
    // let s = s.trim().to_lowercase();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        println!("xxxxxxx {}", s.clone());
        if s == "memory" {
            Ok(Path::Memory)
        }
        // else if s.starts_with("file://") {
        //     Ok(Path::Others(s.replace("file://", "")))
        // } else if s.starts_with("tikv://") {
        //     Ok(Path::Others(s.replace("tikv://", "")))
        // } else if s.starts_with("indxdb://") {
        //     Ok(Path::Others(s.replace("indxdb://", "")))
        // } else if s.starts_with("fdb://") {
        //     Ok(Path::Others(s.replace("fdb://", "")))
        // }
        else {
            Ok(Path::Others(s))
            // Err("Invalid path".to_string())
        }
    }
}

#[derive(Parser, Debug)]
struct SharedAll {
    /// Two way migration
    #[clap(short, long)]
    reversible: bool,

    /// Optional custom migrations dir
    #[clap(short, long)]
    migrations_dir: Option<String>,
}

#[derive(Parser, Debug)]
struct SharedRunAndRollBack {
    #[clap(long)]
    path: Option<Path>,

    #[clap(short, long)]
    db: Option<String>,

    #[clap(short, long)]
    ns: Option<String>,

    /// users scope
    #[clap(short, long)]
    sc: Option<String>,

    #[clap(short, long)]
    user: Option<String>,

    #[clap(short, long)]
    pass: Option<String>,
}

/// Rollback migrations
#[derive(Parser, Debug)]
struct Rollback {
    /// Rollback to the latest migration
    #[clap(long)]
    latest: bool,
    /// Rollback by count
    #[clap(long)]
    by_count: Option<u32>,
    /// Rollback till a specific migration ID
    #[clap(long)]
    till: Option<String>,

    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    shared_run_and_rollback: SharedRunAndRollBack,
}

use std::fmt::Display;
use std::str::FromStr;

// Define the database types and associated values as an enum
// #[derive(ArgEnum, Debug)]
enum DbType {
    Mem,
    RocksDb(String),
    SpeeDb(String),
    FDb(String),
    TiKv(String),
    IndxDb(String),
}

// #[tokio::main] // Or any other runtime you are using
// async fn ain() -> Result<(), Box<dyn std::error::Error>> {
// let opts: Opts = Opts::parse();
//
// match opts.db {
//     DbType::Mem => connect_to_db("Mem", (), "DefaultDB", "DefaultNS").await?,
//     DbType::RocksDb(path) | DbType::SpeeDb(path) | DbType::FDb(path) => {
//         let db_name = opts
//             .database_name
//             .unwrap_or_else(|| String::from("DefaultDB"));
//         let ns_name = opts.namespace.unwrap_or_else(|| String::from("DefaultNS"));
//         connect_to_db("RocksDb", path, &db_name, &ns_name).await?;
//     }
//     DbType::TiKv(url) => {
//         let db_name = opts
//             .database_name
//             .unwrap_or_else(|| String::from("DefaultDB"));
//         let ns_name = opts.namespace.unwrap_or_else(|| String::from("DefaultNS"));
//         connect_to_db("TiKv", url, &db_name, &ns_name).await?;
//     }
//     DbType::IndxDb(name) => {
//         let ns_name = opts.namespace.unwrap_or_else(|| String::from("DefaultNS"));
//         connect_to_db("IndxDb", name, "DefaultDB", &ns_name).await?;
//     }
// }
// Ok(())
// }

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
    // db: Option<Surreal<impl Connection>>,
    user_provided_db: Option<Surreal<Any>>,
    codebase_resources: impl DbResources,
) {
    // todl:
    // for generate,  we dont meed db
    // for run, we need db.
    // for rollback, we need db.
    // user, can provide their own db instance or pass db_url as an rgument to the cli e.g
    // localhost:8000?username=root&password=root&ns=test&db=test
    // if neither is provided, we use panic if running or rolling back, but not for generate
    // we do not need db for generate so we dont panic if db is not provided
    // let db = initialize_db().await;
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
                    .unwrap();
            } else {
                files_config
                    .one_way()
                    .generate_migrations(migration_name, codebase_resources)
                    .await
                    .unwrap();
            };
        }
        SubCommand::Run(run) => {
            let db = setup_db(&user_provided_db, &run.shared_run_and_rollback).await;

            if let Some(path) = run.shared_all.migrations_dir {
                files_config = files_config.custom_path(path)
            };
            if run.shared_all.reversible {
                files_config
                    .two_way()
                    .run_pending_migrations(db.clone())
                    .await
                    .unwrap();
            } else {
                files_config
                    .one_way()
                    .run_pending_migrations(db.clone())
                    .await
                    .unwrap();
            };
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
                .unwrap();
        }
    }
}

async fn setup_db(
    user_provided_db: &Option<Surreal<Any>>,
    shared_run_and_rollback: &SharedRunAndRollBack,
) -> Surreal<Any> {
    let db_url = shared_run_and_rollback.path.clone();
    let db = match (user_provided_db, &db_url) {
        (Some(user_db), None) => {
            let db = user_db;
            init_db(&shared_run_and_rollback, db.clone()).await
        }
        (_, Some(cli_db_url)) => {
            let db = connect(&cli_db_url.to_string()).await.unwrap();
            init_db(&shared_run_and_rollback, db.clone()).await
        }
        (None, None) => panic!("No db provided"),
    };
    db
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
            db.signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .expect("Failed to signin");
        }
    };

    if let Some(db_name) = &shared.db {
        db.use_db("test").await.unwrap();
    }

    if let Some(ns_name) = &shared.ns {
        db.use_ns("test").await.unwrap();
    }
    db
}
