use super::config::{RuntimeConfig, SharedAll};
use clap::Parser;
use std::{fmt::Display, str::FromStr};

use super::config::setup_db;
use crate::{MigrationConfig, MigrationFlag};

#[derive(Clone, Copy, Debug)]
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

impl Status {
    pub fn variants() -> Vec<String> {
        [Status::Applied, Status::Pending, Status::All]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        if s == "pending" {
            Ok(Status::Pending)
        } else if s == "applied" {
            Ok(Status::Applied)
        } else if s == "all" {
            Ok(Status::All)
        } else {
            Err(format!(
                "Invalid status. Must be one of: {}",
                Status::variants().join(", ")
            ))
        }
    }
}

/// Run migrations
#[derive(Parser, Debug)]
pub struct List {
    /// Status of migrations to list
    #[clap(
        long,
        help = "Status of migrations to list. Can be 'applied', 'pending' or 'all'",
        default_value = "applied"
    )]
    pub(crate) status: Option<Status>,

    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,

    #[clap(flatten)]
    pub(crate) runtime_config: RuntimeConfig,
}

impl List {
    pub async fn run(&self) {
        let db = setup_db(&self.runtime_config).await;
        let mut files_config = MigrationConfig::new().make_strict();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.set_custom_path(path)
        };

        match files_config.detect_migration_type() {
            Ok(MigrationFlag::TwoWay) => {
                log::info!("Listing two way migrations");
                let migrations = files_config
                    .two_way()
                    .list_migrations(
                        db.clone(),
                        self.status.unwrap_or(Status::All),
                        self.runtime_config.mode.unwrap_or_default(),
                    )
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
                let migrations = files_config
                    .one_way()
                    .list_migrations(
                        db.clone(),
                        self.status.unwrap_or(Status::All),
                        self.runtime_config.mode.unwrap_or_default(),
                    )
                    .await;

                match migrations {
                    Ok(migrations) => {
                        for migration in migrations {
                            log::info!("Migration name: {migration} ");
                        }
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
