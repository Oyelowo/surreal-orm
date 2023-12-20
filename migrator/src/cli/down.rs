use super::config::{setup_db, RuntimeConfig, SharedAll};
use clap::Parser;

use crate::{MigrationConfig, MigrationFilename, MigrationFlag, RollbackOptions};

/// Rollback migrations
#[derive(Parser, Debug)]
pub struct Down {
    /// Rollback to the latest migration
    #[clap(
        long,
        conflicts_with = "number",
        conflicts_with = "till",
        help = "Rollback to the previous migration"
    )]
    pub(crate) previous: bool,
    /// Rollback by count/number
    #[clap(
        short,
        long,
        conflicts_with = "previous",
        conflicts_with = "till",
        help = "Rollback by count"
    )]
    pub(crate) number: Option<u32>,
    /// Rollback till a specific migration ID
    #[clap(
        short,
        long,
        conflicts_with = "previous",
        conflicts_with = "number",
        help = "Rollback till a specific migration ID"
    )]
    pub(crate) till: Option<String>,

    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,
    #[clap(flatten)]
    pub(crate) shared_run_and_rollback: RuntimeConfig,
}

pub enum RollbackStrategy {
    // Default
    // cargo run -- down
    // cargo run -- down -n 1
    Previous,
    // cargo run -- down -number 34
    // cargo run -- down -n 34
    Number(u32),
    // cargo run -- down -till 234y3498349304
    // cargo run -- down -t 234y3498349304
    Till(MigrationFilename),
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
                    mode: self.shared_run_and_rollback.mode,
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
