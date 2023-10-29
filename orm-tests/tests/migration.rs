use surreal_orm::migrator::{self, embed_migrations};

const MIGRATIONS: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("../migrator/oneway", one_way, strict);

// const MIGRATIONS2: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("../migrator/migrations");
// const MIGRATIONS2: migrator::EmbeddedMigrationsTwoWay =
const MIGRATIONS2: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("../migrator/migrations");

#[test]
fn test_embed_migrations() {
    // insta::assert_display_snapshot!(MIGRATIONS.migrations.to_vec());
}
