use surreal_models::migrations::Resources;
use surreal_orm::migrator::{self, embed_migrations};
use surreal_orm::migrator::{FileManager, MigrationFlag, MigratorDatabase, Mode};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

// Embed migrations as constant
const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("migrations-oneway", one_way, strict);

// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations");
const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!();

async fn generate_one_way_migrations() {
    let file_manager = FileManager {
        mode: Mode::Strict,
        // custom_path: None,
        custom_path: Some("migrations-oneway".to_string()),
        //  Defaults to 'migrations'
        // custom_path: None,
        migration_flag: MigrationFlag::OneWay,
    };

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
        //  Defaults to 'migrations'
        custom_path: None,
        // custom_path: Some("migrations-twoway".to_string()),
        migration_flag: MigrationFlag::TwoWay,
    };

    if let Err(e) =
        MigratorDatabase::generate_migrations("create_new_stuff".into(), &file_manager, Resources)
            .await
    {
        println!("Error: {}", e);
    }
}

async fn generate_default_migrations() {
    let file_manager = FileManager::default();
    MigratorDatabase::generate_migrations("create_new_stuff".into(), &file_manager, Resources)
        .await
        .expect("Failed to generate migrations");
}
#[tokio::main]
async fn main() {
    // GENERATE MIGRATIONS
    // Comment out one of the following to generate migrations
    // generate_two_way_migrations().await;
    // generate_one_way_migrations().await;

    // RUN MIGRATIONS
    println!("Running migrations");
    println!("Embedded migrations: one way {:#?}", MIGRATIONS_ONE_WAY);
    println!("Embedded migrations: two way {:#?}", MIGRATIONS_TWO_WAY);

    // MIGRATIONS_ONE_WAY.run(&Resources).await.unwrap();

    // RUN
    let db = Surreal::new::<Ws>("localhost:8000")
        .await
        .expect("Failed to connect to db");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to signin");
    db.use_ns("test").use_db("test").await.unwrap();
    // let db = Surreal::new::<Mem>(()).await.unwrap();
    // if let Err(e) = Syncer::new(Mode::Strict, db.clone())
    //     .sync_migration(MigrationFlag::OneWay)
    //     .await
    // {
    //     println!("Error: {}", e);
    // }
    // let binding = info_for().database();
    // let x = binding.get_data::<DbInfo>(db.clone()).await.unwrap();
    // println!("Done : {:?}", x);
}
