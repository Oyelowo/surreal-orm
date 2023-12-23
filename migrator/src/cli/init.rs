use super::config::{RuntimeConfig, SharedAll};
use super::up::Up;

use crate::{DbConnection, MigrationConfig, Prompter};

use async_trait::async_trait;
use clap::Parser;
use surreal_query_builder::DbResources;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use typed_builder::TypedBuilder;

/// Init migrations
#[derive(Parser, Debug, TypedBuilder, Clone)]
pub struct Init {
    /// Name of the migration
    #[clap(long, help = "Name of the first migration file(s)")]
    pub(crate) name: String,

    /// Whether or not to run the migrations after initialization.
    #[clap(long)]
    pub(crate) run: bool,

    /// Two way migration
    #[clap(
        short,
        long,
        help = "Unidirectional(Up only) Bidirectional(up & down) migration(S)"
    )]
    pub(crate) reversible: bool,

    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,

    #[clap(flatten)]
    pub(crate) runtime_config: RuntimeConfig,

    #[clap(skip)]
    #[builder(default)]
    pub(crate) db: Option<Surreal<Any>>,
}

impl Init {
    pub async fn run(&self, codebase_resources: impl DbResources, prompter: impl Prompter) {
        let mut files_config = MigrationConfig::new().make_strict();
        let migration_name = self.name.clone();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.set_custom_path(path)
        };
        let files = files_config.clone().get_migrations_filenames(true);

        match files {
            Ok(files) => {
                if !files.is_empty() {
                    log::error!("Migrations already initialized. Run cargo run -- reset to reset migrations");
                    return;
                }
            }
            Err(e) => {
                log::error!("Failed to get migrations: {e}");
                return;
            }
        };

        if self.reversible {
            let gen = files_config
                .two_way()
                .generate_migrations(&migration_name, codebase_resources, prompter)
                .await;
            if let Err(e) = gen {
                log::error!("Failed to generate migrations: {e}");
                return;
            }
        } else {
            let gen = files_config
                .one_way()
                .generate_migrations(migration_name, codebase_resources, prompter)
                .await;

            if let Err(e) = gen {
                log::error!("Failed to generate migrations: {e}");
                return;
            }
        };

        if self.run {
            log::info!("Running initial migrations");

            self.up().run().await;

            log::info!("Successfully ran initial migrations");
        }

        log::info!("Successfully initialized and generated first migration(s)");
    }

    pub fn up(&self) -> Up {
        Up {
            latest: Some(true),
            number: None,
            till: None,
            shared_all: self.shared_all.clone(),
            runtime_config: self.runtime_config.clone(),
            db: self.db.clone(),
        }
    }
}

#[async_trait]
impl DbConnection for Init {
    async fn create_and_set_connection(&mut self) {
        let mut up = self.up();
        up.create_and_set_connection().await;
        self.db = up.db.clone();
    }

    async fn db(&self) -> Surreal<Any> {
        self.db.clone().expect("Failed to get db")
    }
}
