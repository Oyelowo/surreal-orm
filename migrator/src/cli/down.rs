use super::config::{RuntimeConfig, SharedAll};
use async_trait::async_trait;
use clap::Parser;
use surrealdb::{engine::any::Any, Surreal};
use typed_builder::TypedBuilder;

use crate::{
    config::SetupDb, Command, MigrationConfig, MigrationFilename, MigrationFlag, RollbackOptions,
};

/// Rollback migrations
#[derive(Parser, Debug, TypedBuilder)]
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
    pub(crate) runtime_config: RuntimeConfig,
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
        let db = self.db().await;

        if let Ok(MigrationFlag::OneWay) = files_config.detect_migration_type() {
            log::error!(
                "Cannot rollback one way migrations. \
            Create a new migration to reverse the changes or run cargo run -- reset -r \
            to use two way migrations"
            );
            panic!();
        }

        let rollback_strategy = RollbackStrategy::from(self);

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.set_custom_path(path)
        };

        let rollback = files_config
            .two_way()
            .run_down_migrations(
                db.clone(),
                RollbackOptions {
                    rollback_strategy,
                    mode: self.runtime_config.mode.unwrap_or_default(),
                    prune_files_after_rollback: self.runtime_config.prune,
                },
            )
            .await;

        if let Err(ref e) = rollback {
            log::error!("Rollback Failed: {e}");
        } else {
            log::info!("Rollback successful");
        }
    }
}

#[async_trait]
impl Command for Down {
    async fn db(&self) -> Surreal<Any> {
        let db = SetupDb::setup_db(&self.runtime_config).await;
        db.clone()
    }
}
