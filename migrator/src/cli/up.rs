use crate::*;

use clap::Args;
use surreal_query_builder::statements::info_for;
use surreal_query_builder::Runnable;
use typed_builder::TypedBuilder;

/// Run migrations
/// cargo run -- up
/// cargo run -- up -l
/// cargo run -- up -n 2
/// cargo run -- up -t 2021-09-09-xxxxx
#[derive(Args, Debug, TypedBuilder, Clone)]
pub struct Up {
    #[command(flatten)]
    pub(crate) fast_forward: FastForwardDelta,
}

impl Up {
    pub fn update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::from(self)
    }
}

#[derive(Args, Debug, Clone, TypedBuilder)]
#[group(required = false, multiple = false)]
pub struct FastForwardDelta {
    /// Run forward to the latest migration
    #[arg(long, help = "Run forward to the next migration")]
    #[builder(default)]
    pub(crate) latest: bool,

    /// Run forward by count/number
    #[arg(short, long, help = "Run forward by the number specified")]
    #[builder(default, setter(strip_option))]
    pub(crate) number: Option<u32>,

    /// Run forward till a specific migration ID
    #[arg(
        short,
        long,
        value_parser = mig_name_parser,
        help = "Run forward till a specific migration ID"
    )]
    #[builder(default, setter(strip_option))]
    pub(crate) till: Option<MigrationFilename>,
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
        if up.fast_forward.latest {
            UpdateStrategy::Latest
        } else if let Some(by_count) = up.fast_forward.number {
            UpdateStrategy::Number(by_count)
        } else if let Some(till) = up.fast_forward.till.clone() {
            UpdateStrategy::Till(till)
        } else {
            UpdateStrategy::Latest
        }
    }
}

impl Up {
    pub async fn run(&self, cli: &mut Migrator) {
        let file_manager = cli.file_manager();
        let update_strategy = self.update_strategy();
        let db = cli.db().clone();

        match file_manager.detect_migration_type() {
            Ok(MigrationFlag::TwoWay) => {
                log::info!("Running two way migrations");
                let run = file_manager
                    .two_way()
                    .run_up_pending_migrations(db.clone(), update_strategy)
                    .await;
                if let Err(e) = run {
                    log::error!("Failed to run migrations: {e}");
                    panic!("Failed to run migrations due to error: {e}");
                }
            }
            Ok(MigrationFlag::OneWay) => {
                log::info!("Running one way migrations");
                let run = file_manager
                    .one_way()
                    .run_pending_migrations(db.clone(), update_strategy)
                    .await;
                if let Err(e) = run {
                    log::error!("Failed to run migrations: {e}");
                    panic!();
                }
            }
            Err(e) => {
                log::error!("Failed to detect migration type. Make sure the migration  \
                is first initialized or reset by running cargo run -- init -n '<migration name>'. Error: {e}");
                panic!("Failed to detect migration type. Make sure the migration  \
                is first initialized or reset by running cargo run -- init -n '<migration name>'. Error: {e}");
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
