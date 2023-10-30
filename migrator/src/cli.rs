use clap::Parser;
use surreal_query_builder::DbResources;
use surrealdb::engine::remote::ws::Ws;
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
    /// Optional custom migration path
    #[clap(long)]
    optional_custom_path: Option<String>,
    /// Two way migration
    #[clap(short, long)]
    reversible: bool,
    /// Custom Path
    #[clap(long)]
    path: Option<String>,
}

/// Run migrations
#[derive(Parser, Debug)]
struct Run {
    /// Optional custom migration path
    #[clap(long)]
    optional_custom_path: Option<String>,
    /// Enable two way migration
    #[clap(short, long)]
    reversible: bool,
    /// Custom Path
    #[clap(long)]
    path: Option<String>,
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
    /// Optional custom migration path
    #[clap(long)]
    optional_custom_path: Option<String>,
    /// Custom Path
    #[clap(long)]
    path: Option<String>,
}

pub async fn migration_cli(db: Surreal<impl Connection>, codebase_resources: impl DbResources) {
    // let db = initialize_db().await;
    let cli = Cli::parse();

    let mut files_config = MigrationConfig::new().make_strict();
    match cli.subcmd {
        SubCommand::Generate(generate) => {
            let migration_name = generate.name;
            if let Some(path) = generate.path {
                files_config = files_config.custom_path(path)
            };

            if generate.reversible {
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
            if let Some(path) = run.path {
                files_config = files_config.custom_path(path)
            };
            if run.reversible {
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
            if let Some(path) = rollback.path {
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
