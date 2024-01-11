/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use super::up::{FastForwardDelta, Up};
use crate::*;
use clap::Args;
use surreal_query_builder::DbResources;
use typed_builder::TypedBuilder;

/// Init migrations
#[derive(Args, Debug, TypedBuilder, Clone)]
pub struct Init {
    /// Name of the migration
    #[arg(long, help = "Name of the first migration file(s)")]
    pub(crate) name: Basename,

    /// Whether or not to run the migrations after initialization.
    #[arg(long)]
    pub(crate) run: bool,

    /// Two way migration
    #[arg(
        short,
        long,
        help = "Unidirectional(Up only) Bidirectional(up & down) migration(S)"
    )]
    pub(crate) reversible: bool,
}

impl Init {
    pub fn reversible(&self) -> bool {
        self.reversible
    }

    pub async fn run(
        &self,
        cli: &mut Migrator,
        codebase_resources: impl DbResources,
        prompter: impl Prompter,
    ) {
        let migration_name = self.name.clone();
        let file_manager = cli.file_manager();
        let files = file_manager.get_migrations_filenames(true);

        match files {
            Ok(files) => {
                if !files.is_empty() {
                    let err = "Migrations already initialized. Run 'cargo run -- reset' to reset migration. \
                    You can also specify the '-r' or '--reversible' argument to set as reversible. \
                    Or delete the migrations directory and run 'cargo run -- init' again.";
                    log::error!("{err}");
                    panic!("{err}");
                }
            }
            Err(e) => {
                log::error!("Failed to get migrations: {e}");
                panic!("Failed to get migrations: {e}");
            }
        };

        if self.reversible {
            let gen = file_manager
                .two_way()
                .generate_migrations(&migration_name, codebase_resources, prompter)
                .await;
            if let Err(e) = gen {
                log::error!("Failed to generate migrations: {e}");
                panic!("Failed to generate migrations: {e}");
            }
        } else {
            let gen = file_manager
                .one_way()
                .generate_migrations(&migration_name, codebase_resources, prompter)
                .await;

            if let Err(e) = gen {
                log::error!("Failed to generate migrations: {e}");
                panic!("Failed to generate migrations: {e}");
            }
        };

        if self.run {
            cli.setup_db().await;
            log::info!("Running initial migrations");

            self.up().run(cli).await;

            log::info!("Successfully ran initial migrations");
        }

        log::info!("Successfully initialized and generated first migration(s)");
    }

    pub fn up(&self) -> Up {
        Up {
            fast_forward: FastForwardDelta {
                latest: true,
                number: None,
                till: None,
            },
        }
    }
}
