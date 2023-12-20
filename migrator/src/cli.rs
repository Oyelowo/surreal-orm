use std::fmt::Display;
use std::fs;
use std::str::FromStr;

use clap::{ArgAction, Parser};
use surreal_query_builder::statements::info_for;
use surreal_query_builder::{DbResources, Runnable};
use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::{
    Checksum, DbInfo, Migration, MigrationConfig, MigrationFlag, MigrationResult, MigrationRunner,
    RollbackOptions, RollbackStrategy, UpdateStrategy,
};

/// Surreal ORM CLI
#[derive(Parser, Debug)]
#[clap(name = "SurrealOrm", about = "Surreal ORM CLI")]
struct Cli {
    /// Subcommand: generate, up, down, list
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Subcommands
#[derive(Parser, Debug)]
enum SubCommand {
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

impl From<&Up> for UpdateStrategy {
    fn from(up: &Up) -> Self {
        if let Some(true) = up.latest {
            UpdateStrategy::Latest
        } else if let Some(by_count) = up.number {
            UpdateStrategy::Number(by_count)
        } else if let Some(till) = up.till.clone() {
            UpdateStrategy::Till(till.try_into().unwrap())
        } else {
            UpdateStrategy::Latest
        }
    }
}

impl From<&Down> for RollbackStrategy {
    fn from(rollback: &Down) -> Self {
        if rollback.previous {
            RollbackStrategy::Previous
        } else if let Some(by_count) = rollback.number {
            RollbackStrategy::Number(by_count)
        } else if let Some(till) = rollback.till.clone() {
            RollbackStrategy::Till(till.try_into().unwrap())
        } else {
            RollbackStrategy::Previous
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

/// Init migrations
#[derive(Parser, Debug)]
struct Init {
    /// Name of the migration
    #[clap(long, help = "Name of the first migration file(s)")]
    name: String,

    /// Whether or not to run the migrations after initialization.
    #[clap(long)]
    run: bool,

    /// Two way migration
    #[clap(
        short,
        long,
        help = "Unidirectional(Up only) Bidirectional(up & down) migration(S)"
    )]
    reversible: bool,

    #[clap(flatten)]
    shared_all: SharedAll,

    #[clap(flatten)]
    shared_run_and_rollback: RuntimeConfig,
}

impl Init {
    pub async fn run(&self, codebase_resources: impl DbResources) {
        let mut files_config = MigrationConfig::new().make_strict();
        let migration_name = self.name.clone();
        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.custom_path(path)
        };
        let files = files_config
            .clone()
            .into_inner()
            .get_migrations_filenames(true);

        match files {
            Ok(files) => {
                if !files.is_empty() {
                    log::warn!("Migrations already initialized");
                    return ();
                }
            }
            Err(e) => {
                log::error!("Failed to get migrations: {e}");
                panic!();
            }
        };

        if self.reversible {
            let gen = files_config
                .two_way()
                .generate_migrations(&migration_name, codebase_resources)
                .await;
            if let Err(e) = gen {
                log::error!("Failed to generate migrations: {e}");
            }
        } else {
            let gen = files_config
                .one_way()
                .generate_migrations(migration_name, codebase_resources)
                .await;

            if let Err(e) = gen {
                log::error!("Failed to generate migrations: {e}");
            }
        };

        if self.run {
            log::info!("Running initial migrations");

            let run = Up {
                latest: Some(true),
                number: None,
                till: None,
                shared_all: self.shared_all.clone(),
                shared_run_and_rollback: self.shared_run_and_rollback.clone(),
            };
            run.run().await;

            log::info!("Successfully ran initial migrations");
        }

        log::info!("Successfully initialized and generated first migration(s)");
    }
}

/// Generate migrations
#[derive(Parser, Debug)]
struct Generate {
    /// Name of the migration
    #[clap(long, help = "Name of the migration")]
    name: String,

    /// Whether or not to run the migrations after generation.
    #[clap(long, help = "Whether to run the migrations after generation")]
    run: bool,

    #[clap(flatten)]
    shared_all: SharedAll,

    #[clap(flatten)]
    shared_run_and_rollback: RuntimeConfig,
}

impl Generate {
    pub async fn run(&self, codebase_resources: impl DbResources) {
        let mut files_config = MigrationConfig::new().make_strict();
        let migration_name = &self.name;
        let mig_type = files_config.detect_migration_type();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.custom_path(path)
        };

        match mig_type {
            Ok(MigrationFlag::TwoWay) => {
                let gen = files_config
                    .two_way()
                    .generate_migrations(&migration_name, codebase_resources)
                    .await;
                if let Err(e) = gen {
                    log::error!("Failed to generate migrations: {}", e.to_string());
                }
            }
            Ok(MigrationFlag::OneWay) => {
                let gen = files_config
                    .one_way()
                    .generate_migrations(migration_name, codebase_resources)
                    .await;

                if let Err(e) = gen {
                    log::error!("Failed to generate migrations: {}", e.to_string());
                }
            }
            Err(e) => {
                log::error!("Failed to detect migration type: {}", e.to_string());
                panic!();
            }
        };

        if self.run {
            log::info!("Running generated migrations");

            let run = Up {
                latest: Some(true),
                number: None,
                till: None,
                shared_all: self.shared_all.clone(),
                shared_run_and_rollback: self.shared_run_and_rollback.clone(),
            };
            run.run().await;

            log::info!("Successfully ran the generated migration(s)");
        }

        log::info!("Migration generation done.")
    }
}

/// Run migrations
/// cargo run -- up
/// cargo run -- up -l
/// cargo run -- up -n 2
/// cargo run -- up -t 2021-09-09-xxxxx
#[derive(Parser, Debug, Default)]
struct Up {
    /// Run forward to the latest migration
    #[clap(
        long,
        conflicts_with = "number",
        conflicts_with = "till",
        help = "Run forward to the next migration"
    )]
    latest: Option<bool>,
    /// Run forward by count/number
    #[clap(
        short,
        long,
        conflicts_with = "latest",
        conflicts_with = "till",
        help = "Run forward by the number specified"
    )]
    number: Option<u32>,
    /// Run forward till a specific migration ID
    #[clap(
        short,
        long,
        conflicts_with = "latest",
        conflicts_with = "number",
        help = "Run forward till a specific migration ID"
    )]
    till: Option<String>,

    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    shared_run_and_rollback: RuntimeConfig,
}

impl Up {
    pub async fn run(&self) {
        let db = setup_db(&self.shared_run_and_rollback).await;
        let update_strategy = UpdateStrategy::from(self);
        let mut files_config = MigrationConfig::new().make_strict();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.custom_path(path)
        }

        match files_config.detect_migration_type() {
            Ok(MigrationFlag::TwoWay) => {
                log::info!("Running two way migrations");
                let run = files_config
                    .two_way()
                    .run_up_pending_migrations(db.clone(), update_strategy)
                    .await;
                if let Err(e) = run {
                    log::error!("Failed to run migrations: {}", e.to_string());
                    panic!();
                }
            }
            Ok(MigrationFlag::OneWay) => {
                log::info!("Running one way migrations");
                let run = files_config
                    .one_way()
                    .run_pending_migrations(db.clone(), update_strategy)
                    .await;
                if let Err(e) = run {
                    log::error!("Failed to run migrations: {}", e.to_string());
                    panic!();
                }
            }
            Err(e) => {
                log::error!("Failed to detect migration type: {}", e.to_string());
                panic!();
            }
        };

        let info = info_for().database().get_data::<DbInfo>(db.clone()).await;
        if let Err(ref e) = info {
            log::error!("Failed to get db info: {}", e.to_string());
        }

        log::info!("Successfully ran migrations");
        log::info!("Database: {:?}", info);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Status {
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

impl List {
    pub async fn run(&self) {
        let db = setup_db(&self.runtime_config).await;
        let mut files_config = MigrationConfig::new().make_strict();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.custom_path(path)
        };

        match files_config.detect_migration_type() {
            Ok(MigrationFlag::TwoWay) => {
                log::info!("Listing two way migrations");
                let migrations = files_config
                    .two_way()
                    .list_migrations(
                        db.clone(),
                        self.status.unwrap_or(Status::All),
                        self.runtime_config.strict.into(),
                    )
                    .await;

                match migrations {
                    Ok(migrations) => {
                        log::info!("Listing {} migrations.", migrations.len());
                        log::info!("=================================================");
                        for migration in migrations {
                            log::info!("{migration} ");
                        }
                        log::info!("=================================================");
                        log::info!("Listing end.");
                    }
                    Err(ref e) => {
                        log::error!("Failed to get migrations: {e}");
                    }
                }
            }
            Ok(MigrationFlag::OneWay) => {
                log::info!("Listing one way migrations");
                let migrations = files_config
                    .one_way()
                    .list_migrations(
                        db.clone(),
                        self.status.unwrap_or(Status::All),
                        self.runtime_config.strict.into(),
                    )
                    .await;

                match migrations {
                    Ok(migrations) => {
                        for migration in migrations {
                            log::info!("Migration name: {migration} ");
                        }
                    }
                    Err(ref e) => {
                        log::error!("Failed to get migrations: {e}");
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to detect migration type: {}", e.to_string());
            }
        };
    }
}

/// Delete Unapplied local migration files that have not been applied to the current database instance
/// cargo run -- prune
#[derive(Parser, Debug)]
struct Prune {
    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    runtime_config: RuntimeConfig,
}

impl Prune {
    pub async fn run(&self) {
        let mut files_config = MigrationConfig::new().make_strict();
        let db = setup_db(&self.runtime_config).await;
        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.custom_path(path)
        }

        let res =
            MigrationRunner::delete_unapplied_migration_files(db.clone(), &files_config.relax())
                .await;

        if let Err(ref e) = res {
            log::error!("Failed to prune migrations: {}", e.to_string());
            panic!();
        }

        log::info!("Prune successful");
    }
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

#[derive(Parser, Debug, Default, Clone)]
struct SharedAll {
    /// Optional custom migrations dir
    #[clap(short, long, help = "Optional custom migrations dir")]
    migrations_dir: Option<String>,

    /// Sets the level of verbosity e.g -v, -vv, -vvv, -vvvv
    #[clap(short, long, action = ArgAction::Count, default_value="3")]
    verbose: u8,
}

#[derive(Parser, Debug, Clone)]
struct RuntimeConfig {
    /// URL or path to connect to a database instance. Supports various backends.
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
        // value_name = "URL",
        default_value = "ws://localhost:8000",
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
    url: Path,

    #[clap(long, default_value = "test", help = "Database name")]
    db: Option<String>,

    #[clap(long, default_value = "test", help = "Namespace name")]
    ns: Option<String>,

    /// users scope
    #[clap(long, help = "Scope")]
    sc: Option<String>,

    #[clap(short, long, default_value = "root", help = "User name")]
    user: Option<String>,

    #[clap(short, long, default_value = "root", help = "Password")]
    pass: Option<String>,

    #[clap(
        long,
        help = "If to be strict or lax. Strictness validates the migration files against the database e.g doing checksum checks to make sure.\
        that file contents and valid and also checking filenames. Lax does not.",
        default_value = "true"
    )]
    strict: bool,

    #[clap(
        long,
        help = "If to prune migration files after rollback",
        default_value = "false"
    )]
    prune: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig::parse()
    }
}

/// Rollback migrations
#[derive(Parser, Debug)]
struct Down {
    /// Rollback to the latest migration
    #[clap(
        long,
        conflicts_with = "number",
        conflicts_with = "till",
        help = "Rollback to the previous migration"
    )]
    previous: bool,
    /// Rollback by count/number
    #[clap(
        short,
        long,
        conflicts_with = "previous",
        conflicts_with = "till",
        help = "Rollback by count"
    )]
    number: Option<u32>,
    /// Rollback till a specific migration ID
    #[clap(
        short,
        long,
        conflicts_with = "previous",
        conflicts_with = "number",
        help = "Rollback till a specific migration ID"
    )]
    till: Option<String>,

    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    shared_run_and_rollback: RuntimeConfig,
}

impl Down {
    pub async fn run(&self) {
        let mut files_config = MigrationConfig::new().make_strict();

        if let Ok(MigrationFlag::OneWay) = files_config.detect_migration_type() {
            log::error!(
                "Cannot rollback one way migrations. \
            Create a new migration to reverse the changes or run cargo run -- reset -r \
            to use two way migrations"
            );
            panic!();
        }

        let db = setup_db(&self.shared_run_and_rollback).await;
        let rollback_strategy = RollbackStrategy::from(self);

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.custom_path(path)
        };

        let rollback = files_config
            .two_way()
            .run_down_migrations(
                db.clone(),
                RollbackOptions {
                    rollback_strategy,
                    strictness: self.shared_run_and_rollback.strict.into(),
                    prune_files_after_rollback: self.shared_run_and_rollback.prune,
                },
            )
            .await;

        if let Err(ref e) = rollback {
            log::error!("Failed to rollback migrations: {e}");
        }

        log::info!("Rollback successful");
    }
}

/// Resets migrations. Deletes all migration files, migration table and reinitializes
/// migrations.
#[derive(Parser, Debug)]
struct Reset {
    /// Name of the first migration file(s) to reinitialize to
    #[clap(long)]
    name: String,

    /// Whether or not to run the migrations after reinitialization. Reinitalization
    /// is done by deleting all migration files, and regenerating
    /// the first migration file(s) which include queries to delete all old
    /// migration metadata in the database before creating the new ones.
    #[clap(long)]
    run: bool,

    /// Two way migration
    #[clap(
        short,
        long,
        help = "Whether to reinitialize as Unidirectional(Up only) Bidirectional(up & down) migration(S)"
    )]
    reversible: bool,
    #[clap(flatten)]
    shared_all: SharedAll,
    #[clap(flatten)]
    shared_run_and_rollback: RuntimeConfig,
}

impl Reset {
    pub async fn run(&self, codebase_resources: impl DbResources) {
        let mut files_config = MigrationConfig::new().make_strict();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.custom_path(path)
        };

        let dir = files_config.get_migration_dir_create_if_none();
        match dir {
            Ok(dir) => {
                if dir.exists() {
                    let removed = fs::remove_dir_all(&dir);
                    if let Err(e) = removed {
                        log::error!("Failed to remove dir: {e}");
                        panic!();
                    } else {
                        fs::create_dir(&dir).expect("Problem creating migration directory");
                        log::info!("Migration directory recreated.");
                    }
                } else {
                    fs::create_dir(dir).expect("Problem creating migration directory");
                    log::info!("Migration directory recreated.");
                }
            }
            Err(e) => {
                log::error!("Failed to get migration dir: {e}");
                panic!();
            }
        };

        let init = Init {
            name: self.name.clone(),
            run: self.run,
            reversible: self.reversible.clone(),
            shared_all: self.shared_all.clone(),
            shared_run_and_rollback: self.shared_run_and_rollback.clone(),
        };
        init.run(codebase_resources).await;

        log::info!("Reset successful");
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

    match cli.subcmd {
        SubCommand::Init(init) => {
            init.run(codebase_resources).await;
        }
        SubCommand::Generate(generate) => {
            generate.run(codebase_resources).await;
        }
        SubCommand::Up(up) => {
            up.run().await;
        }
        SubCommand::Down(down) => {
            down.run().await;
        }
        SubCommand::Prune(prune) => {
            prune.run().await;
        }
        SubCommand::List(prune) => {
            prune.run().await;
        }
        SubCommand::Reset(reset) => {
            reset.run(codebase_resources).await;
        }
    }
}

async fn setup_db(shared_run_and_rollback: &RuntimeConfig) -> Surreal<Any> {
    let cli_db_url = shared_run_and_rollback.url.clone();
    let db = connect(&cli_db_url.to_string()).await.unwrap();
    init_db(shared_run_and_rollback, db.clone()).await
}

async fn init_db(shared: &RuntimeConfig, db: Surreal<Any>) -> Surreal<Any> {
    match (&shared.user, &shared.pass) {
        (Some(u), Some(p)) => {
            let signin = db
                .signin(Root {
                    username: u.as_str(),
                    password: p.as_str(),
                })
                .await;
            if let Err(e) = signin {
                log::error!("Failed to signin: {}", e.to_string());
                panic!();
            }
            log::info!("Signed in successfully");
        }
        (Some(_), None) => {
            log::error!("Password not provided");
            panic!();
        }
        (None, Some(_)) => {
            log::error!("User not provided");
            panic!();
        }
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
        let db = db.use_db(db_name).await;
        if let Err(e) = db {
            log::error!("Failed to use db: {}", e.to_string());
            panic!();
        }
    }

    if let Some(ns_name) = &shared.ns {
        log::info!("Using ns {}", ns_name);
        let ns = db.use_ns(ns_name).await;
        if let Err(e) = ns {
            log::error!("Failed to use ns: {}", e.to_string());
            panic!();
        }
    }
    db
}
