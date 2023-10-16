use m::{Database, DbInfo, Direction, Migration, Planet, Student};
use migrator as m;
use surreal_orm::{
    statements::{begin_transaction, info_for},
    transaction, Buildable, Model, Node, Raw, Runnable, SurrealCrudNode, ToRaw,
};

#[tokio::main]
async fn main() {
    m::Database::run_migrations().await;
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

    // let statement1 = Planet::define_fields()
    //     .iter()
    //     .map(|f| f.to_raw().build())
    //     .collect::<Vec<_>>();
    //
    // let statement2 = Student::define_fields()
    //     .iter()
    //     .map(|f| f.to_raw().build())
    //     .collect::<Vec<_>>();
    //
    // let queries = [statement1, statement2].concat();
    // // let queries = vec![statement1, statement2];
    // // let direction = m::Direction::get_up_migration(&self);
    // let name = "create_users_table";
    //
    // // From Migration directory
    // let db = m::Database::init().await;
    // db.run_migrations_in_local_dir().await.unwrap();
    // let db_info = db.get_db_info().await.unwrap();
    // println!("db info: {:#?}", db_info);
    // let table_info = db.get_table_info("planet".into()).await.unwrap();
    //
    // println!("table info: {:#?}", table_info.get_fields_definitions());
    //
    // // From Codebase
    // let db2 = m::Database::init().await;
    // db2.run_codebase_schema_queries().await.unwrap();
    // let db_info2 = db2.get_db_info().await.unwrap();
    // println!("db info2: {:#?}", db_info2);
    //
    // let table_info2 = db2.get_table_info("eats".into()).await.unwrap();
    //
    // println!("table info2: {:#?}", table_info2.get_fields_definitions());
    // let migs = m::Migration::get_all_from_migrations_dir();
    // println!("migs: {:#?}", migs);
}

// UPDATE person SET firstName = none;
// #
// # REMOVE FIELD firstName ON TABLE person;
//
//
// fn get_code_base_db_info() -> CodeBaseDbInfo {
//     let fields = HashMap::new();
//     tables.insert(
//         "planet".to_string(),
//         Planet::define_fields()
//             .iter()
//             .map(|f| f.to_raw().build())
//             .collect::<Vec<_>>(),
//     );
//     tables.insert(
//         "student".to_string(),
//         Student::define_fields()
//             .iter()
//             .map(|f| f.to_raw().build())
//             .collect::<Vec<_>>(),
//     );
//     let tables = HashMap::new();
//     tables.insert("planet".to_string(), Planet::define_table());
//     tables.insert("student".to_string(), Student::define_table());
//
//     let db_info = DbInfo {
//         tables,
//         ..Default::default()
//     };
// }
//
//

//  DIFFING
//  LEFT
//
// Left = migration directory
// Right = codebase
//
// 1. Get all migrations from migration directory synced with db - Left
// 2. Get all migrations from codebase synced with db - Right
// 3. Diff them
//
// // For Tables (Can probably use same heuristics for events, indexes, analyzers, functions,
// params, etc)
// a. If there a Table in left that is not in right,
//     (i) up => REMOVE TABLE table_name;
//     (ii) down => DEFINE TABLE table_name; (Use migration directory definition)
//
// b. If there a Table in right that is not in left,
//    (i) up => DEFINE TABLE table_name; (Use codebase definition)
//    (ii) down => REMOVE TABLE table_name;
//
// c. If there a Table in left and in right,
//   (i) up => Use Right table definitions(codebase definition)
//   (ii) down => Use Left table definitions(migration directory definition)
//
// For Fields
//  a. If there a Field in left that is not in right,
//          (i) up => REMOVE FIELD
//          (ii) down => ADD FIELD
//  b. If there a Field in right that is not in left,
//        (i) up => ADD FIELD
//        (ii) down => REMOVE FIELD
//  c. If there a Field in left and in right,
//      (i) up => Use Right field definitions
//      (ii) down => Use Left field definitions
//  d. If there is a field name change,
//    Get old and new names. Surrealdb does not support Alter statement
//    (i) up =>
//              DEFINE FIELD new_name on TABLE table_name;
//              UPDATE table_name SET new_name = old_name;
//              REMOVE old_name on TABLE table_name; or UPDATE table_name SET old_name = none;
//    (ii) down =>
//          DEFINE FIELD old_name on TABLE table_name;
//          UPDATE table_name SET old_name = new_name;
//          REMOVE new_name on TABLE table_name; or UPDATE table_name SET new_name = none;
//
//o
//
// 4. Aggregate all the new up and down queries
// 5. Run the queries as a transaction
// 6. Update the migration directory with the new migrations queries
// // m::create_migration_file(up, down, name);
// 7. Mark the queries as registered

// Run the diff
// 5. Update the migration directory
//
