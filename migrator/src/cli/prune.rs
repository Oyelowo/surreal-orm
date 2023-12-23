use super::config::{RuntimeConfig, SharedAll};

use async_trait::async_trait;
use clap::Parser;
use surrealdb::{engine::any::Any, Surreal};
use typed_builder::TypedBuilder;

use crate::{config::SetupDb, DbConnection, MigrationConfig, MigrationRunner};

/// Delete Unapplied local migration files that have not been applied to the current database instance
/// cargo run -- prune
#[derive(Parser, Debug, TypedBuilder, Clone)]
pub struct Prune {
    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,

    #[clap(flatten)]
    pub(crate) runtime_config: RuntimeConfig,

    #[clap(skip)]
    pub(crate) db: Option<Surreal<Any>>,
}

impl Prune {
    pub async fn run(&self) {
        let mut files_config = MigrationConfig::new().make_strict();
        let db = self.db().await;
        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.set_custom_path(path)
        }

        let res =
            MigrationRunner::delete_unapplied_migration_files(db.clone(), &files_config.relax())
                .await;

        if let Err(ref e) = res {
            log::error!("Failed to prune migrations: {e}");
            panic!();
        }

        log::info!("Prune successful");
    }
}

#[async_trait]
impl DbConnection for Prune {
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
