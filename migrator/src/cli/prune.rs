use super::config::{RuntimeConfig, SharedAll};

use async_trait::async_trait;
use clap::Parser;
use surrealdb::{engine::any::Any, Surreal};
use typed_builder::TypedBuilder;

use crate::{Cli, DbConnection, MigrationConfig, MigrationRunner};

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
    pub async fn run(&self, cli: &mut Cli) {
        let file_manager = cli.file_manager();
        let db = cli.db().clone();

        let res =
            MigrationRunner::delete_unapplied_migration_files(db.clone(), &file_manager.relax())
                .await;

        if let Err(ref e) = res {
            log::error!("Failed to prune migrations: {e}");
            panic!();
        }

        log::info!("Prune successful");
    }
}
