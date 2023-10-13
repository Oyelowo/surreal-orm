use m::{Database, Planet, Student};
use migrator as m;
use surreal_orm::{statements::info_for, Buildable, Model, Node, Runnable, SurrealCrudNode, ToRaw};

#[tokio::main]
async fn main() {
    // let statement1 = Planet::default().create().to_raw().build();
    // let statement2 = Planet {
    //     name: "Earth".to_string(),
    //     population: 7_000_000_000,
    //     tags: vec!["rocky".to_string(), "habitable".to_string()],
    //     ..Default::default()
    // }
    // .create()
    // .to_raw()
    // .build();

    let statement1 = Planet::define_fields()
        .iter()
        .map(|f| f.to_raw().build())
        .collect::<Vec<_>>();

    let statement2 = Student::define_fields()
        .iter()
        .map(|f| f.to_raw().build())
        .collect::<Vec<_>>();

    let queries = [statement1, statement2].concat();
    // let queries = vec![statement1, statement2];
    // let direction = m::Direction::get_up_migration(&self);
    let direction = m::Direction::new(queries, None);
    let name = "create_users_table";
    // m::create_migration_file(direction, name);

    //
    // let queries = all_migrations
    //     .iter()
    //     .map(|m| m.direction.get_up_migration())
    //     .collect::<Vec<_>>()
    //     .join("\n");

    let queries = direction.get_up_migration();
    // Run them as a transaction
    // From DB
    let db = m::Database::init().db().await;

    let query_to_run = format!("BEGIN TRANSACTION; {queries} COMMIT TRANSACTION; ");
    println!("query_to_run: {:#?}", query_to_run);
    db.query(query_to_run).await;
    // ------

    // --- Get metadata from in-memory db
    // db.execute(query_to_run).await;

    let db_info = m::Database::get_db_info(db.clone()).await;
    println!("tables: {:#?}", db_info.get_tables());

    let table_info = m::Database::get_table_info(db.clone(), "planet".into()).await;
    println!("table field defs: {:#?}", table_info.get_fields());
    println!("table field names: {:#?}", table_info.get_fields_names());
    println!(
        "table field defs: {:#?}",
        table_info.get_fields_definitions()
    );
    let table_info = m::Database::get_table_info(db.clone(), "student".into()).await;
    println!("table field defs: {:#?}", table_info.get_fields());
    println!("table field names: {:#?}", table_info.get_fields_names());
    println!(
        "table field defs: {:#?}",
        table_info.get_fields_definitions()
    );
    // Get all tables from queries within the directory using a parser.

    // ###########
    // From migrations directory
    // let all_migrations = m::get_all_migrations_from_dir();
    // println!("Hello, world!, {all_migrations:#?}");

    // get all scehma from codebase
}
