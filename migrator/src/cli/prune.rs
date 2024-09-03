/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use crate::*;
use clap::Args;

/// Delete Unapplied local migration files that have not been applied to the current database instance
/// cargo run -- prune
#[derive(Args, Debug, Clone)]
pub struct Prune;

impl Prune {
    pub async fn run(&self, cli: &mut Migrator) {
        cli.setup_db().await;
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
