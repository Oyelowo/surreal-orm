use surreal_models::migrations::Resources;
use surreal_orm::migrator::{self, embed_migrations};
use surreal_orm::migrator::{FileManager, MigrationFlag, MigratorDatabase, Mode};

// Embed migrations as constant
const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("migrations-oneway", one_way, strict);
const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations");

async fn generate_one_way_migrations() {
    let file_manager = FileManager {
        mode: Mode::Strict,
        // custom_path: None,
        custom_path: Some("migrations-oneway".to_string()),
        //  Defaults to 'migrations'
        // custom_path: None,
        migration_flag: MigrationFlag::OneWay,
    };
    // let file_manager = FileManager::default();
    if let Err(e) =
        MigratorDatabase::generate_migrations("create_new_stuff".into(), &file_manager, Resources)
            .await
    {
        println!("Error: {}", e);
    }
}

async fn generate_two_way_migrations() {
    let file_manager = FileManager {
        mode: Mode::Strict,
        // custom_path: None,
        custom_path: Some("migrations-oneway".to_string()),
        //  Defaults to 'migrations'
        // custom_path: None,
        migration_flag: MigrationFlag::OneWay,
    };
    // let file_manager = FileManager::default();
    if let Err(e) =
        MigratorDatabase::generate_migrations("create_new_stuff".into(), &file_manager, Resources)
            .await
    {
        println!("Error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    // GENERATE MIGRATIONS
    // Comment out one of the following to generate migrations
    // generate_two_way_migrations().await;
    // generate_one_way_migrations().await;
}
