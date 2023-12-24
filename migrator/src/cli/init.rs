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
    pub(crate) name: String,

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
    pub async fn run(
        &self,
        cli: &mut Migrator,
        codebase_resources: impl DbResources,
        prompter: impl Prompter,
    ) {
        // let mut files_config = MigrationConfig::new().make_strict();
        let migration_name = self.name.clone();

        let file_manager = cli.file_manager();
        let files = file_manager.get_migrations_filenames(true);

        match files {
            Ok(files) => {
                if !files.is_empty() {
                    log::error!("Migrations already initialized. Run 'cargo run -- reset' to reset migration. \
                    You can also specify the '-r' or '--reversible' argument to set as reversible. \
                    Or delete the migrations directory and run 'cargo run -- init' again.");
                    return;
                }
            }
            Err(e) => {
                log::error!("Failed to get migrations: {e}");
                return;
            }
        };

        if self.reversible {
            let gen = file_manager
                .two_way()
                .generate_migrations(&migration_name, codebase_resources, prompter)
                .await;
            if let Err(e) = gen {
                log::error!("Failed to generate migrations: {e}");
                return;
            }
        } else {
            let gen = file_manager
                .one_way()
                .generate_migrations(migration_name, codebase_resources, prompter)
                .await;

            if let Err(e) = gen {
                log::error!("Failed to generate migrations: {e}");
                return;
            }
        };

        if self.run {
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
