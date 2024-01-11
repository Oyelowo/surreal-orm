/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use clap::Args;
use typed_builder::TypedBuilder;

use crate::*;

/// Rollback migrations
#[derive(Args, Debug, TypedBuilder, Clone)]
pub struct Down {
    #[command(flatten)]
    strategy: RollbackStrategyStruct,

    #[arg(
        global = true,
        long,
        help = "If to prune migration files after rollback",
        default_value_t = false
    )]
    pub(crate) prune: bool,
}

impl Down {
    pub fn rollback_strategy(&self) -> RollbackStrategy {
        RollbackStrategy::from(&self.strategy)
    }
}

#[derive(Args, Debug, Clone, TypedBuilder)]
#[group(required = false, multiple = false)]
pub struct RollbackStrategyStruct {
    /// Rollback to the previous migration
    #[arg(long, help = "Rollback to the previous migration")]
    #[builder(default)]
    pub(crate) previous: bool,

    /// Rollback by count/number
    #[arg(short, long, help = "Rollback by the number specified")]
    #[builder(default, setter(strip_option))]
    pub(crate) number: Option<u32>,

    /// Rollback till a specific migration ID
    #[arg(
        short,
        long,
        value_parser = mig_name_parser,
        help = "Rollback till a specific migration ID"
    )]
    #[builder(default, setter(strip_option))]
    pub(crate) till: Option<MigrationFilename>,
}

impl Default for RollbackStrategyStruct {
    fn default() -> Self {
        RollbackStrategyStruct {
            previous: true,
            number: None,
            till: None,
        }
    }
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

impl From<&RollbackStrategyStruct> for RollbackStrategy {
    fn from(strategy: &RollbackStrategyStruct) -> Self {
        if strategy.previous {
            RollbackStrategy::Previous
        } else if let Some(by_count) = strategy.number {
            RollbackStrategy::Number(by_count)
        } else if let Some(by_name) = strategy.till.clone() {
            RollbackStrategy::Till(by_name)
        } else {
            RollbackStrategy::Previous
        }
    }
}

impl Down {
    pub async fn run(&self, cli: &mut Migrator) {
        cli.setup_db().await;
        let file_manager = cli.file_manager();
        let db = cli.db().clone();

        if let Ok(MigrationFlag::OneWay) = file_manager.detect_migration_type() {
            let err = "Cannot rollback one way migrations. \
            Create a new migration to reverse the changes or run cargo run -- reset -r \
            to use two way migrations";

            log::error!("{err}");
            panic!("{err}");
        }

        let rollback_strategy = self.rollback_strategy();

        let rollback = file_manager
            .two_way()
            .run_down_migrations(
                db.clone(),
                RollbackOptions {
                    rollback_strategy,
                    mode: cli.mode,
                },
            )
            .await;

        if let Err(ref e) = rollback {
            log::error!("Rollback Failed: {e}");
            panic!("Rollback Failed: {e}");
        } else {
            log::info!("Rollback successful");

            if self.prune {
                log::info!("Pruning all pending migration files");
                self.prune().run(cli).await;
                log::info!("Pruning successful");
            }

            log::info!("Rollback Ddne");
        }
    }

    fn prune(&self) -> Prune {
        Prune
    }
}
