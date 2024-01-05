/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use clap::Args;
use std::fs;
use typed_builder::TypedBuilder;

use crate::*;
use surreal_query_builder::DbResources;

/// Resets migrations. Deletes all migration files, migration table and reinitializes
/// migrations.
#[derive(Args, Debug, TypedBuilder, Clone)]
pub struct Reset {
    /// Name of the first migration file(s) to reinitialize to
    #[arg(long)]
    pub(crate) name: Basename,

    /// Whether or not to run the migrations after reinitialization. Reinitalization
    /// is done by deleting all migration files, and regenerating
    /// the first migration file(s) which include queries to delete all old
    /// migration metadata in the database before creating the new ones.
    #[arg(long)]
    pub(crate) run: bool,

    /// Two way migration
    #[arg(
        short,
        long,
        help = "Whether to reinitialize as Unidirectional(Up only) Bidirectional(up & down) migration(S)"
    )]
    pub(crate) reversible: bool,
}

impl Reset {
    pub fn reversible(&self) -> bool {
        self.reversible
    }

    pub async fn run(
        &self,
        cli: &mut Migrator,
        codebase_resources: impl DbResources,
        prompter: impl Prompter,
    ) {
        let file_manager = cli.file_manager();
        let dir = file_manager.get_migration_dir_create_if_none();

        match dir {
            Ok(dir) => {
                if dir.exists() {
                    let removed = fs::remove_dir_all(&dir);
                    if let Err(e) = removed {
                        log::error!("Failed to remove dir: {e}");
                        panic!();
                    } else {
                        fs::create_dir(&dir).expect("Problem creating migration directory");
                        log::info!("Migration directory recreated.");
                    }
                } else {
                    fs::create_dir(dir).expect("Problem creating migration directory");
                    log::info!("Migration directory recreated.");
                }
            }
            Err(e) => {
                log::error!("Failed to get migration dir: {e}");
                panic!();
            }
        };

        self.init_command()
            .run(cli, codebase_resources, prompter)
            .await;

        log::info!("Reset successful");
    }

    fn init_command(&self) -> Init {
        Init {
            name: self.name.clone(),
            run: self.run,
            reversible: self.reversible.clone(),
        }
    }
}
