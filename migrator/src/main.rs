use m::{Database, Direction, Migration, Planet, Student};
use migrator as m;
use surreal_orm::{
    statements::{begin_transaction, info_for},
    transaction, Buildable, Model, Node, Raw, Runnable, SurrealCrudNode, ToRaw,
};

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
    let name = "create_users_table";
    // m::create_migration_file(direction.clone(), name);

    //
    // let queries = all_migrations
    //     .iter()
    //     .map(|m| m.direction.get_up_migration())
    //     .collect::<Vec<_>>()
    //     .join("\n");
    let up_queries = queries.join(";\n");
    // let down_queries = queries.join(";\n");
    let down_queries =
        "REMOVE fake_field on table person;\nREMOVE fake_name on table person".to_string();
    // Migration::create_migration_file(up_queries, Some(down_queries), name);
    // let query_to_run = format!("BEGIN TRANSACTION; {queries} COMMIT TRANSACTION; ");
    // println!("query: {:#?}", query);

    // Run them as a transaction
    // From DB
    let db = m::Database::init().await;
    db.run_migrations_in_local_dir().await.unwrap();
    let db_info = db.get_db_info().await.unwrap();
    println!("db info: {:#?}", db_info);

    let table_info = db.get_table_info("planet".into()).await.unwrap();

    println!("table info: {:#?}", table_info.get_fields_definitions());
    // let migs = m::Migration::get_all_from_migrations_dir();
    // println!("migs: {:#?}", migs);
}

// UPDATE person SET firstName = none;
// #
// # REMOVE FIELD firstName ON TABLE person;
