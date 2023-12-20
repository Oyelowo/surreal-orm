use super::config::{RuntimeConfig, SharedAll};
use super::up::Up;

use crate::{MigrationConfig, MigrationFlag};
use clap::Parser;
use surreal_query_builder::DbResources;

/// Generate migrations
#[derive(Parser, Debug)]
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
    pub(crate) shared_run_and_rollback: RuntimeConfig,
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
