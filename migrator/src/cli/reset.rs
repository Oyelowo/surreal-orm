use super::config::{RuntimeConfig, SharedAll};

use async_trait::async_trait;
use clap::Parser;
use std::fs;
use surrealdb::{engine::any::Any, Surreal};
use typed_builder::TypedBuilder;

use surreal_query_builder::DbResources;

use super::init::Init;
use crate::{config::SetupDb, Command, MigrationConfig, Prompter};

/// Resets migrations. Deletes all migration files, migration table and reinitializes
/// migrations.
#[derive(Parser, Debug, TypedBuilder)]
pub struct Reset {
    /// Name of the first migration file(s) to reinitialize to
    #[clap(long)]
    pub(crate) name: String,

    /// Whether or not to run the migrations after reinitialization. Reinitalization
    /// is done by deleting all migration files, and regenerating
    /// the first migration file(s) which include queries to delete all old
    /// migration metadata in the database before creating the new ones.
    #[clap(long)]
    pub(crate) run: bool,

    /// Two way migration
    #[clap(
        short,
        long,
        help = "Whether to reinitialize as Unidirectional(Up only) Bidirectional(up & down) migration(S)"
    )]
    pub(crate) reversible: bool,

    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,

    #[clap(flatten)]
    pub(crate) runtime_config: RuntimeConfig,

    #[clap(skip)]
    pub(crate) db: Option<Surreal<Any>>,
}

impl Reset {
    pub async fn run(&self, codebase_resources: impl DbResources, prompter: impl Prompter) {
        let mut files_config = MigrationConfig::new().make_strict();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.set_custom_path(path)
        };

        let dir = files_config.get_migration_dir_create_if_none();
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

        self.init_command().run(codebase_resources, prompter).await;

        log::info!("Reset successful");
    }

    fn init_command(&self) -> Init {
        Init {
            name: self.name.clone(),
            run: self.run,
            reversible: self.reversible.clone(),
            shared_all: self.shared_all.clone(),
            runtime_config: self.runtime_config.clone(),
            db: self.db.clone(),
        }
    }
}

#[async_trait]
impl Command for Reset {
    async fn create_and_set_connection(&mut self) {
        // let db = SetupDb::new(&self.runtime_config).await.clone();
        // self.db = Some(db.clone());
    }

    async fn db(&self) -> Surreal<Any> {
        self.init_command().db().await
    }
}
