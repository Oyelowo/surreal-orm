use surreal_models::migrations::Resources;
use surreal_orm::migrator::{FileManager, MigrationFlag, MigratorDatabase, Mode};
use surreal_orm::migrator::{self, embed_migrations};

// const MIGRATIONS: migrator::EmbeddedMigrationsOneWay =
//     embed_migrations!("migrations", one_way, strict);

// const MIGRATIONS2: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("../migrator/migrations");
// const MIGRATIONS2: migrator::EmbeddedMigrationsTwoWay =
const MIGRATIONS2: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("kaod");


#[tokio::main]
async fn main() {
    // GENERATE MIGRATIONS
    let file_manager = FileManager {
        mode: Mode::Strict,
        custom_path: Some("../kaod".to_string()),
        //  Defaults to 'migrations'
        // custom_path: None,
        migration_flag: MigrationFlag::TwoWay,
    };
    let file_manager = FileManager::default();
    if let Err(e) =
        MigratorDatabase::generate_migrations("create_new_stuff".into(), &file_manager, Resources)
            .await
    {
        println!("Error: {}", e);
    }
}
