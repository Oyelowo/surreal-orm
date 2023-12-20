use super::config::{RuntimeConfig, SharedAll};
use super::up::Up;

use clap::{ArgAction, Parser};
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

use surreal_query_builder::statements::info_for;
use surreal_query_builder::{DbResources, Runnable};
use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::{DbInfo, MigrationConfig, MigrationFlag, MigrationRunner, RollbackOptions};

/// Init migrations
#[derive(Parser, Debug)]
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
    pub(crate) shared_run_and_rollback: RuntimeConfig,
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
