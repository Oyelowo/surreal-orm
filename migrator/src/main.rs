use std::fmt::Display;

use inquire::InquireError;
use m::{Database, DbInfo, LeftDatabase, MigrationFlag, Mode, Planet, Student, Syncer};
use migrator as m;
use surreal_orm::{
    statements::{begin_transaction, info_for},
    transaction, Buildable, Model, Node, Raw, Runnable, SurrealCrudNode, ToRaw,
};
use surrealdb::{
    engine::remote::ws::Ws,
    opt::auth::Root,
    sql::{
        statements::{DefineStatement, DefineTokenStatement},
        Base, Statement,
    },
    Surreal,
};

#[tokio::main]
async fn main() {
    // GENERATE MIGRATIONS
    // if let Err(e) = m::Database::generate_migrations(&"create_new_stuff".to_string(), true).await {
    //     println!("Error: {}", e);
    // }

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
    if let Err(e) = Syncer::new(Mode::Strict, db.clone())
        .sync_migration(MigrationFlag::OneWay)
        .await
    {
        println!("Error: {}", e);
    }
    let binding = info_for().database();
    let x = binding.get_data::<DbInfo>(db.clone()).await.unwrap();
    println!("Done : {:?}", x);
}
