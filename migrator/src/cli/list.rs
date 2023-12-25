use clap::{Args, ValueEnum};
use std::fmt::Display;
use typed_builder::TypedBuilder;

use crate::*;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Status {
    Applied,
    Pending,
    All,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Applied => write!(f, "applied"),
            Status::Pending => write!(f, "pending"),
            Status::All => write!(f, "all"),
        }
    }
}

/// Run migrations
#[derive(Args, Debug, TypedBuilder, Clone)]
pub struct List {
    /// Status of migrations to list
    #[arg(
        value_enum,
        long,
        help = "Status of migrations to list. Can be 'applied', 'pending' or 'all'",
        default_value_t = Status::Applied
    )]
    pub(crate) status: Status,
}

impl List {
    pub async fn run(&self, cli: &mut Migrator) {
        let db = cli.db().clone();
        let file_manager = cli.file_manager();

        match file_manager.detect_migration_type() {
            Ok(MigrationFlag::TwoWay) => {
                log::info!("Listing two way migrations");

                let migrations = file_manager
                    .two_way()
                    .list_migrations(db.clone(), self.status, cli.mode)
                    .await;

                match migrations {
                    Ok(migrations) => {
                        log::info!("Listing {} migrations.", migrations.len());
                        log::info!("=================================================");
                        for migration in migrations {
                            log::info!("{migration} ");
                        }
                        log::info!("=================================================");
                        log::info!("Listing end.");
                    }
                    Err(ref e) => {
                        log::error!("Failed to get migrations: {e}");
                    }
                }
            }
            Ok(MigrationFlag::OneWay) => {
                log::info!("Listing one way migrations");
                let migrations = file_manager
                    .one_way()
                    .list_migrations(db.clone(), self.status, cli.mode)
                    .await;

                match migrations {
                    Ok(migrations) => {
                        log::info!("=================================================");
                        for migration in migrations {
                            log::info!("Migration name: {migration} ");
                        }
                        log::info!("=================================================");
                        log::info!("Listing end.");
                    }
                    Err(ref e) => {
                        log::error!("Failed to get migrations: {e}");
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to detect migration type: {}", e.to_string());
            }
        };
    }
}
