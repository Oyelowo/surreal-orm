/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use surreal_orm::migrator::{
    self, config::DatabaseConnection, embed_migrations, Migration, Mode, UpdateStrategy,
};

// Embed migrations as constant
const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("tests/migrations-oneway", one_way, strict);

const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay =
    embed_migrations!("tests/migrations-twoway", two_way, strict);

#[test]
fn test_embedded() {
    assert_eq!(MIGRATIONS_ONE_WAY.get_migrations().len(), 2);
    assert_eq!(MIGRATIONS_TWO_WAY.get_migrations().len(), 1);

    let migs = MIGRATIONS_ONE_WAY.to_migrations_one_way().unwrap();
    assert_eq!(migs.len(), 2);
    // check the meta data
    assert_eq!(
        migs[0].name().to_string(),
        "20231029202315_create_new_stuff.surql"
    );

    assert_eq!(
        migs[0].name().basename(),
        "create_new_stuff".to_string().try_into().unwrap()
    );
    insta::assert_display_snapshot!(migs[0].content());
    assert_eq!(
        migs[1].name().to_string(),
        "20231029224601_create_another.surql"
    );
    assert_eq!(migs[1].name().basename(), "create_another".into());
    assert_eq!(migs.len(), 2);
    insta::assert_display_snapshot!(migs[1].content());

    let migs = MIGRATIONS_TWO_WAY.to_migrations_two_way().unwrap();
    assert_eq!(migs.len(), 1);

    // check the meta data
    assert_eq!(
        migs[0].up.name.to_string(),
        "20231030025711_migration_name_example.up.surql"
    );
    assert_eq!(migs[0].up.name.basename(), "migration_name_example".into());
    insta::assert_display_snapshot!(migs[0].up.content);
    insta::assert_display_snapshot!(migs[0].down.content);
}

#[tokio::test]
async fn test_embedded_run_one_way() {
    let db = DatabaseConnection::default().setup().await.db().unwrap();

    MIGRATIONS_ONE_WAY
        .run(db.clone(), UpdateStrategy::Latest, Mode::Strict)
        .await
        .unwrap();

    let db_migrations_meta = Migration::get_all_desc(db.clone()).await;
    insta::assert_debug_snapshot!(db_migrations_meta);
    assert_eq!(db_migrations_meta.len(), 2);
}

#[tokio::test]
async fn test_embedded_run_two_way() {
    let db = DatabaseConnection::default().setup().await.db().unwrap();

    MIGRATIONS_TWO_WAY
        .run(db.clone(), UpdateStrategy::Latest, Mode::Strict)
        .await
        .unwrap();

    let db_migrations_meta = Migration::get_all_desc(db.clone()).await;
    insta::assert_debug_snapshot!(db_migrations_meta);
    assert_eq!(db_migrations_meta.len(), 1);
}
