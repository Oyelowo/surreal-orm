use super::config::{RuntimeConfig, SharedAll};
use super::up::Up;

use crate::config::SetupDb;
use crate::{DbConnection, MigrationConfig, MigrationFlag, Prompter, RealPrompter};
use async_trait::async_trait;
use clap::Parser;
use surreal_query_builder::DbResources;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use typed_builder::TypedBuilder;

/// Generate migrations
#[derive(Parser, Debug, TypedBuilder)]
pub struct Generate {
    /// Name of the migration
    #[clap(long, help = "Name of the migration")]
    pub(crate) name: String,

    /// Whether or not to run the migrations after generation.
    #[clap(long, help = "Whether to run the migrations after generation")]
    pub(crate) run: bool,

    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,

    #[clap(flatten)]
    pub(crate) runtime_config: RuntimeConfig,

    #[clap(skip)]
    pub(crate) db: Option<Surreal<Any>>,
}

impl Generate {
    pub async fn run(&self, codebase_resources: impl DbResources, prompter: impl Prompter) {
        let mut files_config = MigrationConfig::new().make_strict();
        let migration_name = &self.name;
        let mig_type = files_config.detect_migration_type();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.set_custom_path(path)
        };

        match mig_type {
            Ok(MigrationFlag::TwoWay) => {
                let gen = files_config
                    .two_way()
                    .generate_migrations(&migration_name, codebase_resources, prompter)
                    .await;
                if let Err(e) = gen {
                    log::error!("Failed to generate migrations: {e}");
                }
            }
            Ok(MigrationFlag::OneWay) => {
                let gen = files_config
                    .one_way()
                    .generate_migrations(migration_name, codebase_resources, prompter)
                    .await;

                if let Err(e) = gen {
                    log::error!("Failed to generate migrations: {e}");
                }
            }
            Err(e) => {
                log::error!("Failed to detect migration type: {e}");
                panic!();
            }
        };

        if self.run {
            log::info!("Running generated migrations");

            self.up().run().await;

            log::info!("Successfully ran the generated migration(s)");
        }

        log::info!("Migration generation done.");
        self.up().run().await;
    }

    fn up(&self) -> Up {
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
impl DbConnection for Generate {
    async fn create_and_set_connection(&mut self) {
        // let db = SetupDb::new(&self.runtime_config).await.clone();
        // self.db = Some(db.clone());
    }

    async fn db(&self) -> Surreal<Any> {
        self.up().db().await
    }
}
