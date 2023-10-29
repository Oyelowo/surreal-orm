use surreal_orm::migrator::{self, embed_migrations};

const MIGRATIONS: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("../migrator/oneway", one_way, strict);

// const x: Vec<migrator::MigrationOneWay> = embed_migrations!("../migrator/oneway", one_way, strict);
#[test]
fn test_embed_migrations() {

    // let x = include_str!("../../migrator/oneway/20231027223423_create_new_stuff.surql");
    // let x = include_str!("../Cargo.toml");
    // embed_migrations!();
    // let x: Vec<migrator::MigrationOneWay> =
    //     embed_migrations!("../migrator/oneway", one_way, strict);

    // embed_migrations!("", one_way, strict);
    // embed_migrations!(oneway, one_way, strict);
}
