/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{
    Alien, AlienWithExplicitAttributes, StudentWithDefineAttr, StudentWithDefineFnAttr,
    StudentWithGranularAttributes,
};
use surreal_orm::*;
use surrealdb::{engine::local::Mem, Surreal};

#[tokio::test]
async fn test_node_type_atttribute_auto_inferred() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();

    db.use_ns("test").use_db("test").await.unwrap();
    assert_eq!(
        Alien::define_table().to_raw().build(),
        "DEFINE TABLE alien;"
    );

    assert_eq!(
        Alien::define_fields()
            .iter()
            .map(|x| x.to_raw().build())
            .collect::<Vec<_>>()
            .join("\n"),
        "DEFINE FIELD id ON TABLE alien TYPE record<alien>;
DEFINE FIELD name ON TABLE alien TYPE string;
DEFINE FIELD age ON TABLE alien TYPE int;
DEFINE FIELD created ON TABLE alien TYPE datetime;
DEFINE FIELD lifeExpectancy ON TABLE alien TYPE duration;
DEFINE FIELD lineString ON TABLE alien TYPE geometry<LineString>;
DEFINE FIELD territoryArea ON TABLE alien TYPE geometry<polygon>;
DEFINE FIELD home ON TABLE alien TYPE geometry<point>;
DEFINE FIELD tags ON TABLE alien TYPE array<string>;
DEFINE FIELD ally ON TABLE alien TYPE record<alien>;
DEFINE FIELD weapon ON TABLE alien TYPE record<weapon>;
DEFINE FIELD spaceShips ON TABLE alien TYPE array<record<space_ship>>;
DEFINE FIELD planetsToVisit ON TABLE alien TYPE any;"
    );

    Ok(())
}

#[tokio::test]
async fn test_node_type_atttribute_explicit() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    assert_eq!(
        AlienWithExplicitAttributes::define_table().to_raw().build(),
        "DEFINE TABLE alien_with_explicit_attributes;"
    );

    assert_eq!(
        AlienWithExplicitAttributes::define_fields()
            .iter()
            .map(|x| x.to_raw().build())
            .collect::<Vec<_>>()
            .join("\n"),
        "DEFINE FIELD id ON TABLE alien_with_explicit_attributes TYPE record<alien_with_explicit_attributes>;
DEFINE FIELD name ON TABLE alien_with_explicit_attributes TYPE string;
DEFINE FIELD age ON TABLE alien_with_explicit_attributes TYPE int;
DEFINE FIELD created ON TABLE alien_with_explicit_attributes TYPE datetime;
DEFINE FIELD lifeExpectancy ON TABLE alien_with_explicit_attributes TYPE duration;
DEFINE FIELD territoryArea ON TABLE alien_with_explicit_attributes TYPE geometry<polygon>;
DEFINE FIELD home ON TABLE alien_with_explicit_attributes TYPE geometry<point>;
DEFINE FIELD tags ON TABLE alien_with_explicit_attributes TYPE array<string>;
DEFINE FIELD ally ON TABLE alien_with_explicit_attributes TYPE record<alien_with_explicit_attributes>;
DEFINE FIELD weapon ON TABLE alien_with_explicit_attributes TYPE record<weapon>;
DEFINE FIELD spaceShips ON TABLE alien_with_explicit_attributes TYPE array<record<space_ship>>;"
    );

    Ok(())
}

#[tokio::test]
async fn test_node_attributes_() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    insta::assert_snapshot!(StudentWithGranularAttributes::define_table()
        .to_raw()
        .build());

    insta::assert_snapshot!(StudentWithGranularAttributes::define_fields()
        .iter()
        .map(|x| x.to_raw().build())
        .collect::<Vec<_>>()
        .join("\n"));

    Ok(())
}

#[tokio::test]
async fn test_node_attributes_with_define_attribute() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    insta::assert_snapshot!(StudentWithDefineAttr::define_table().to_raw().build());

    insta::assert_snapshot!(StudentWithDefineAttr::define_fields()
        .iter()
        .map(|x| x.to_raw().build())
        .collect::<Vec<_>>()
        .join("\n"));

    Ok(())
}

#[tokio::test]
async fn test_node_attributes_with_define_fn_attribute() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    insta::assert_snapshot!(StudentWithDefineFnAttr::define_table().to_raw().build());

    insta::assert_snapshot!(StudentWithDefineFnAttr::define_fields()
        .iter()
        .map(|x| x.to_raw().build())
        .collect::<Vec<_>>()
        .join("\n"));

    Ok(())
}
