use clap::Args;
use typed_builder::TypedBuilder;

use crate::*;

/// Rollback migrations
#[derive(Args, Debug, TypedBuilder, Clone)]
pub struct Down {
    #[command(flatten)]
    strategy: RollbackDelta,
}

impl Down {
    pub fn rollback_strategy(&self) -> RollbackStrategy {
        RollbackStrategy::from(self)
    }
}

#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = false)]
pub struct RollbackDelta {
    /// Rollback to the previous migration
    #[arg(long, help = "Rollback to the previous migration")]
    pub(crate) previous: bool,

    /// Rollback by count/number
    #[arg(short, long, help = "Rollback by the number specified")]
    pub(crate) number: Option<u32>,

    /// Rollback till a specific migration ID
    #[arg(
        short,
        long,
        value_parser = mig_name_parser,
        help = "Rollback till a specific migration ID"
    )]
    pub(crate) till: Option<MigrationFilename>,
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
        if rollback.strategy.previous {
            RollbackStrategy::Previous
        } else if let Some(by_count) = rollback.strategy.number {
            RollbackStrategy::Number(by_count)
        } else if let Some(by_name) = rollback.strategy.till.clone() {
            RollbackStrategy::Till(by_name)
        } else {
            RollbackStrategy::Previous
        }
    }
}

impl Down {
    pub async fn run(&self, cli: &mut Migrator) {
        let file_manager = cli.file_manager();
        let db = cli.db().clone();

        if let Ok(MigrationFlag::OneWay) = file_manager.detect_migration_type() {
            log::error!(
                "Cannot rollback one way migrations. \
            Create a new migration to reverse the changes or run cargo run -- reset -r \
            to use two way migrations"
            );
            panic!();
        }

        let rollback_strategy = self.rollback_strategy();

        let rollback = file_manager
            .two_way()
            .run_down_migrations(
                db.clone(),
                RollbackOptions {
                    rollback_strategy,
                    mode: cli.runtime_config.mode,
                    prune_files_after_rollback: cli.runtime_config.prune,
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
