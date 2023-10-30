use clap::Parser;
use surreal_models::migrations::Resources;
use surreal_orm::migrator::{self, embed_migrations, MigrationConfig, RollbackStrategy};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

async fn initialize_db() -> Surreal<surrealdb::engine::remote::ws::Client> {
    let db = Surreal::new::<Ws>("localhost:8000")
        .await
        .expect("Failed to connect to db");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to signin");
    db.use_ns("test").use_db("test").await.unwrap();
    db
}

#[tokio::main]
async fn main() {
    let db = initialize_db().await;
    let cli = Cli::parse();

    // ONE WAY MIGRATIONS
    let mut files_config = MigrationConfig::new().make_strict();

    let one_way = files_config.custom_path("migrations-oneway").one_way();
    // Comment out this line to generate oneway migrations
    // To be used from cli
    one_way
        .generate_migrations("migration_name_example", Resources)
        .await
        .unwrap();

    // Run normal non-embedded pending migrations in migration directory
    one_way.run_pending_migrations(db.clone()).await.unwrap();

    // TWO WAY MIGRATIONS
    let two_way = files_config.custom_path("migrations-twoway").two_way();

    // GENERATE MIGRATIONS
    // comment out this line to generate twoway migrations
    // To be used from cli
    two_way
        .generate_migrations("migration_name_example", Resources)
        .await
        .unwrap();

    // two_way
    //     .rollback_migrations(RollbackStrategy::Latest, db.clone())
    //     // .rollback_migrations(RollbackStrategy::ByCount(4), db.clone())
    //     // .rollback_migrations(
    //     //     RollbackStrategy::UntilMigrationFileName("name".to_string().try_into().unwrap()),
    //     //     db.clone(),
    //     // )
    //     .await
    //     .unwrap();

    // Run normal non-embedded pending migrations in migration directory
    two_way.run_pending_migrations(db.clone()).await.unwrap();
}

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
}

#[tokio::main]
async fn mainx() {
    let db = initialize_db().await;
    let cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Generate(generate) => {
            let mut files_config = MigrationConfig::new().make_strict();
            let migration_type = if generate.reversible {
                "migrations-twoway"
            } else {
                "migrations-oneway"
            };
            let generate_path = generate
                .optional_custom_path
                .unwrap_or_else(|| migration_type.to_string());

            let generator = files_config.custom_path(&generate_path);
            generator
                .generate_migrations(&generate.name, Resources)
                .await
                .unwrap();
        }
        SubCommand::Run(run) => {
            let mut files_config = MigrationConfig::new().make_strict();
            let migration_type = if run.reversible {
                "migrations-twoway"
            } else {
                "migrations-oneway"
            };
            let run_path = run
                .optional_custom_path
                .unwrap_or_else(|| migration_type.to_string());

            let runner = files_config.custom_path(&run_path);
            runner.run_pending_migrations(db.clone()).await.unwrap();
        }
        SubCommand::Rollback(rollback) => {
            let mut files_config = MigrationConfig::new().make_strict();
            let migration_type = if rollback.reversible {
                "migrations-twoway"
            } else {
                "migrations-oneway"
            };
            let rollback_path = rollback
                .optional_custom_path
                .unwrap_or_else(|| migration_type.to_string());

            let rollbacker = files_config.custom_path(&rollback_path);

            if rollback.latest {
                // Implement logic to rollback to the latest migration
            } else if let Some(count) = rollback.by_count {
                // Implement logic to rollback by count
            } else if let Some(migration_id) = rollback.till {
                // Implement logic to rollback till a specific migration ID
            }
        }
    }
}
