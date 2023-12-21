use super::config::{RuntimeConfig, SharedAll};

use clap::Parser;
use surrealdb::{engine::any::Any, Surreal};

use crate::{config::SetupDb, MigrationConfig, MigrationRunner};

/// Delete Unapplied local migration files that have not been applied to the current database instance
/// cargo run -- prune
#[derive(Parser, Debug)]
pub struct Prune {
    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,

    #[clap(flatten)]
    pub(crate) runtime_config: RuntimeConfig,
}

impl Prune {
    pub async fn run(&self, db_setup: &mut SetupDb) -> Surreal<Any> {
        let mut files_config = MigrationConfig::new().make_strict();
        let setup = db_setup.override_runtime_config(&self.runtime_config);
        let db = setup.db();
        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.set_custom_path(path)
        }

        let res =
            MigrationRunner::delete_unapplied_migration_files(db.clone(), &files_config.relax())
                .await;

        if let Err(ref e) = res {
            log::error!("Failed to prune migrations: {}", e.to_string());
            panic!();
        }

        log::info!("Prune successful");
        db.clone()
    }
}
