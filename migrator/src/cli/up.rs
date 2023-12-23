use super::config::{RuntimeConfig, SharedAll};
use crate::config::SetupDb;
use crate::{Command, DbInfo, MigrationConfig, MigrationFilename, MigrationFlag};

use async_trait::async_trait;
use clap::Parser;
use surreal_query_builder::statements::info_for;
use surreal_query_builder::Runnable;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use typed_builder::TypedBuilder;

/// Run migrations
/// cargo run -- up
/// cargo run -- up -l
/// cargo run -- up -n 2
/// cargo run -- up -t 2021-09-09-xxxxx
#[derive(Parser, Debug, TypedBuilder)]
pub struct Up {
    /// Run forward to the latest migration
    #[clap(
        long,
        conflicts_with = "number",
        conflicts_with = "till",
        help = "Run forward to the next migration"
    )]
    pub(crate) latest: Option<bool>,

    /// Run forward by count/number
    #[clap(
        short,
        long,
        conflicts_with = "latest",
        conflicts_with = "till",
        help = "Run forward by the number specified"
    )]
    pub(crate) number: Option<u32>,

    /// Run forward till a specific migration ID
    #[clap(
        short,
        long,
        conflicts_with = "latest",
        conflicts_with = "number",
        help = "Run forward till a specific migration ID"
    )]
    pub(crate) till: Option<String>,

    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,

    #[clap(flatten)]
    pub(crate) runtime_config: RuntimeConfig,

    #[clap(skip)]
    pub(crate) db: Option<Surreal<Any>>,
}

pub enum UpdateStrategy {
    // Default
    // cargo run -- up
    // cargo run -- up --latest
    // cargo run -- up -l
    Latest,
    // cargo run -- up --number 34
    // cargo run -- up -n 34
    Number(u32),
    // cargo run -- up --till 234y3498349304
    // cargo run -- up -t 234y3498349304
    Till(MigrationFilename),
}

impl From<&Up> for UpdateStrategy {
    fn from(up: &Up) -> Self {
        if let Some(true) = up.latest {
            UpdateStrategy::Latest
        } else if let Some(by_count) = up.number {
            UpdateStrategy::Number(by_count)
        } else if let Some(till) = up.till.clone() {
            UpdateStrategy::Till(till.try_into().expect("Failed to parse migration filename"))
        } else {
            UpdateStrategy::Latest
        }
    }
}

pub trait RunnableMigration {
    fn runtime_config(&self) -> &RuntimeConfig;
}

impl RunnableMigration for Up {
    fn runtime_config(&self) -> &RuntimeConfig {
        &self.runtime_config
    }
}

impl Up {
    pub async fn run(&self) {
        // create_and_set_connection
        let db = self.db().await;

        let update_strategy = UpdateStrategy::from(self);
        let mut files_config = MigrationConfig::new().make_strict();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.set_custom_path(path)
        }

        match files_config.detect_migration_type() {
            Ok(MigrationFlag::TwoWay) => {
                log::info!("Running two way migrations");
                let run = files_config
                    .two_way()
                    .run_up_pending_migrations(db.clone(), update_strategy)
                    .await;
                if let Err(e) = run {
                    log::error!("Failed to run migrations: {e}");
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
                    log::error!("Failed to run migrations: {e}");
                    panic!();
                }
            }
            Err(e) => {
                log::error!("Failed to detect migration type: {e}");
                panic!();
            }
        };

        let info = info_for().database().get_data::<DbInfo>(db.clone()).await;
        if let Err(ref e) = info {
            log::error!("Failed to get db info: {e}");
        }

        log::info!("Successfully ran migrations");
        log::info!("Database: {:?}", info);
    }
}

#[async_trait]
impl Command for Up {
    async fn create_and_set_connection(&mut self) {
        let db = SetupDb::new(&self.runtime_config).await.clone();
        if self.db.is_none() {
            self.db = Some(db.clone());
        }
    }

    async fn db(&self) -> Surreal<Any> {
        self.db.clone().expect("Failed to get db")
    }
}
