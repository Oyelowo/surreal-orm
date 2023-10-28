use migrator_macros::embed_migrations;

#[test]
fn test_embed_migrations() {
    let x = include_str!("../../migrator/oneway/20231027223423_create_new_stuff.surql");
    let x = include_str!("../Cargo.toml");
    embed_migrations!(
        // "../../migrator/oneway/20231027223423_create_new_stuff.surql",
        "../migrator/oneway",
        one_way,
        strict
    );
    // embed_migrations!("", one_way, strict);
    // embed_migrations!(oneway, one_way, strict);
}
