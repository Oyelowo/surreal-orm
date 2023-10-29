use surreal_orm::migrator::{self, embed_migrations};

const MIGRATIONS: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("../migrator/oneway", one_way, strict);

//     surreal_orm::migrator::EmbeddedMigrationsOneWay::new(&[surreal_orm::migrator::EmbeddedMigrationOneWay {
//   id:"20231027223423_create_new_stuff.surql",
//   name:"20231027223423_create_new_stuff",
//   timestamp:20231027223423u64,
//   content:"DEFINE TABLE planet SCHEMAFULL;\nDEFINE FIELD population ON planet TYPE int;\nDEFINE FIELD id ON planet TYPE record<planet>;\nDEFINE FIELD tags ON planet TYPE array;\nDEFINE FIELD updatedAt ON planet TYPE datetime;\nDEFINE FIELD firstName ON planet TYPE string;\nDEFINE FIELD createdAt ON planet TYPE datetime;\nDEFINE TABLE student SCHEMAFULL;\nDEFINE FIELD updatedAt ON student TYPE datetime;\nDEFINE FIELD createdAt ON student TYPE datetime;\nDEFINE FIELD university ON student TYPE string;\nDEFINE FIELD age ON student TYPE int;\nDEFINE FIELD id ON student TYPE record<student>;\nDEFINE TABLE animal SCHEMAFULL;\nDEFINE FIELD id ON animal TYPE record<animal>;\nDEFINE FIELD species ON animal TYPE string;\nDEFINE FIELD createdAt ON animal TYPE datetime;\nDEFINE FIELD velocity ON animal TYPE int;\nDEFINE FIELD updatedAt ON animal TYPE datetime;\nDEFINE FIELD attributes ON animal TYPE array;\nDEFINE INDEX species_speed_idx ON animal FIELDS species, velocity UNIQUE;\nDEFINE EVENT event1 ON animal WHEN species = 'Homo Erectus' AND velocity > 545 THEN (SELECT * FROM crop);\nDEFINE EVENT event2 ON animal WHEN (species = 'Homo Sapien') AND (velocity < 10) THEN (SELECT * FROM eats);\nDEFINE TABLE crop SCHEMAFULL;\nDEFINE FIELD color ON crop TYPE string;\nDEFINE FIELD id ON crop TYPE record<crop>;\nDEFINE TABLE eats SCHEMAFULL;\nDEFINE FIELD place ON eats TYPE string;\nDEFINE FIELD in ON eats TYPE record;\nDEFINE FIELD out ON eats TYPE record;\nDEFINE FIELD createdAt ON eats TYPE datetime;\nDEFINE FIELD id ON eats TYPE record<eats>;",
// },surreal_orm::migrator::EmbeddedMigrationOneWay {
//   id:"20231028000344_create_new_stuff.surql",name:"20231028000344_create_new_stuff",timestamp:20231028000344u64,content:"DEFINE TABLE migration SCHEMAFULL;\nDEFINE FIELD timestamp ON migration TYPE int;\nDEFINE FIELD id ON migration TYPE record<migration>;\nDEFINE FIELD name ON migration TYPE string;",
// }])
// //
// const x: Vec<migrator::MigrationOneWay> = embed_migrations!("../migrator/oneway", one_way, strict);
#[test]
fn test_embed_migrations() {
    // MIGRATIONS
    // MIGRATIONS.migrations.
    let xx =     surreal_orm::migrator::EmbeddedMigrationsOneWay::new(&[surreal_orm::migrator::EmbeddedMigrationOneWay {
  id:"20231027223423_create_new_stuff.surql",
        name:"20231027223423_create_new_stuff",
        timestamp:20231027223423u64,
        content: "DEFINE TABLE planet SCHEMAFULL;\nDEFINE FIELD population ON planet TYPE int;\nDEFINE FIELD id ON planet TYPE record<planet>;\nDEFINE FIELD tags ON planet TYPE array;\nDEFINE FIELD updatedAt ON planet TYPE datetime;\nDEFINE FIELD firstName ON planet TYPE string;\nDEFINE FIELD createdAt ON planet TYPE datetime;\nDEFINE TABLE student SCHEMAFULL;\nDEFINE FIELD updatedAt ON student TYPE datetime;\nDEFINE FIELD createdAt ON student TYPE datetime;\nDEFINE FIELD university ON student TYPE string;\nDEFINE FIELD age ON student TYPE int;\nDEFINE FIELD id ON student TYPE record<student>;\nDEFINE TABLE animal SCHEMAFULL;\nDEFINE FIELD id ON animal TYPE record<animal>;\nDEFINE FIELD species ON animal TYPE string;\nDEFINE FIELD createdAt ON animal TYPE datetime;\nDEFINE FIELD velocity ON animal TYPE int;\nDEFINE FIELD updatedAt ON animal TYPE datetime;\nDEFINE FIELD attributes ON animal TYPE array;\nDEFINE INDEX species_speed_idx ON animal FIELDS species, velocity UNIQUE;\nDEFINE EVENT event1 ON animal WHEN species = 'Homo Erectus' AND velocity > 545 THEN (SELECT * FROM crop);\nDEFINE EVENT event2 ON animal WHEN (species = 'Homo Sapien') AND (velocity < 10) THEN (SELECT * FROM eats);\nDEFINE TABLE crop SCHEMAFULL;\nDEFINE FIELD color ON crop TYPE string;\nDEFINE FIELD id ON crop TYPE record<crop>;\nDEFINE TABLE eats SCHEMAFULL;\nDEFINE FIELD place ON eats TYPE string;\nDEFINE FIELD in ON eats TYPE record;\nDEFINE FIELD out ON eats TYPE record;\nDEFINE FIELD createdAt ON eats TYPE datetime;\nDEFINE FIELD id ON eats TYPE record<eats>;",
},surreal_orm::migrator::EmbeddedMigrationOneWay {
  id:"20231028000344_create_new_stuff.surql",name:"20231028000344_create_new_stuff",timestamp:20231028000344u64,content:"DEFINE TABLE migration SCHEMAFULL;\nDEFINE FIELD timestamp ON migration TYPE int;\nDEFINE FIELD id ON migration TYPE record<migration>;\nDEFINE FIELD name ON migration TYPE string;",
}]);

    // let x = include_str!("../../migrator/oneway/20231027223423_create_new_stuff.surql");
    // let x = include_str!("../Cargo.toml");
    // embed_migrations!();
    // let x: Vec<migrator::MigrationOneWay> =
    //     embed_migrations!("../migrator/oneway", one_way, strict);

    // embed_migrations!("", one_way, strict);
    // embed_migrations!(oneway, one_way, strict);
}
