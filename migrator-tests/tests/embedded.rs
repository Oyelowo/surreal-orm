/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use surreal_orm::migrator::{self, embed_migrations};

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
        "20231029224601_create_new_stuff.surql"
    );
    assert_eq!(migs[1].name().basename(), "create_new_stuff".into());
    assert_eq!(migs.len(), 2);
    assert_eq!(
        migs[1].content().to_string(),
        "DEFINE FIELD labels ON planet TYPE array;\nUPDATE planet SET labels = tags;\nREMOVE FIELD tags ON TABLE planet;".to_string()
    );

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
