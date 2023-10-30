/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use migrator::{
    FileManager, MigrationConfig, MigrationFlag, MigrationRunner, MigratorDatabase, Mode,
};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[tokio::main]
async fn main() {
    // GENERATE MIGRATIONS
    let db = initialize_db().await;

    // One way migrations
    // Make sure migrations directory exists or anyone you set in the config
    // before running this
    let mut files_config = MigrationConfig::new();

    MigrationRunner::run_pending_migrations(
        files_config.one_way().get_migrations().unwrap(),
        db.clone(),
    )
    .await
    .unwrap();

    // Two way migrations
    MigrationRunner::run_pending_migrations(
        files_config.two_way().get_migrations().unwrap(),
        db.clone(),
    )
    .await
    .unwrap();
}

async fn initialize_db() -> Surreal<Client> {
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
    db
}
