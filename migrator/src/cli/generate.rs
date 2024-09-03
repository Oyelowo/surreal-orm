/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use super::up::{FastForwardDelta, Up};

use crate::*;
use clap::Args;
use surreal_query_builder::DbResources;
use typed_builder::TypedBuilder;

/// Generate migrations
#[derive(Args, Debug, TypedBuilder, Clone)]
pub struct Generate {
    /// Name of the migration
    #[arg(long, help = "Name of the migration")]
    pub(crate) name: Basename,

    /// Whether or not to run the migrations after generation.
    #[arg(long, help = "Whether to run the migrations after generation")]
    #[builder(default)]
    pub(crate) run: bool,
}

impl Generate {
    pub async fn run(
        &self,
        cli: &mut Migrator,
        codebase_resources: impl DbResources,
        prompter: impl Prompter,
    ) {
        let file_manager = cli.file_manager();
        let migration_basename = &self.name;
        let mig_type = file_manager.detect_migration_type();

        match mig_type {
            Ok(MigrationFlag::TwoWay) => {
                log::info!("Generating two-way migration");
                let gen = file_manager
                    .two_way()
                    .generate_migrations(migration_basename, codebase_resources, prompter)
                    .await;
                if let Err(e) = gen {
                    log::error!("Failed to generate migrations: {e}");
                    panic!("Failed to generate migrations");
                }
            }
            Ok(MigrationFlag::OneWay) => {
                let gen = file_manager
                    .one_way()
                    .generate_migrations(migration_basename, codebase_resources, prompter)
                    .await;

                if let Err(e) = gen {
                    log::error!("Failed to generate migrations: {e}");
                    panic!("Failed to generate migrations");
                }
            }
            Err(e) => {
                log::error!("Failed to detect migration type: {e}");
                panic!("Failed to detect migration type");
            }
        };

        if self.run {
            cli.setup_db().await;
            log::info!("Running generated migrations");

            self.up().run(cli).await;

            log::info!("Successfully ran the generated migration(s)");
        }

        log::info!("Migration generation done.");
    }

    fn up(&self) -> Up {
        Up {
            fast_forward: FastForwardDelta {
                latest: true,
                number: None,
                till: None,
            },
        }
    }
}
