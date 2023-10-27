/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use migrator::{FileManager, MigrationFlag, MigratorDatabase, Resources};

#[tokio::main]
async fn main() {
    // GENERATE MIGRATIONS
    // let file_manager = FileManager {
    //     mode: migrator::Mode::Strict,
    //     // custom_path: Some("banff"),
    //     //  Defaults to 'migrations'
    //     custom_path: None,
    //     migration_flag: MigrationFlag::OneWay,
    // };
    let file_manager = FileManager::default();
    if let Err(e) =
        MigratorDatabase::generate_migrations("create_new_stuff".into(), &file_manager, Resources)
            .await
    {
        println!("Error: {}", e);
    }

    // RUN
    // let db = Surreal::new::<Ws>("localhost:8000")
    //     .await
    //     .expect("Failed to connect to db");
    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await
    // .expect("Failed to signin");
    // db.use_ns("test").use_db("test").await.unwrap();
    // // let db = Surreal::new::<Mem>(()).await.unwrap();
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
