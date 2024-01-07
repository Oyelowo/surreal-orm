/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{
    AlienVisitsPlanet, AlienVisitsPlanetExplicit, AlienVisitsPlanetWithExplicitAttributes,
};
use surreal_orm::*;
use surrealdb::{engine::local::Mem, Surreal};

#[tokio::test]
async fn test_node_atttributes_auto_inferred() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();

    db.use_ns("test").use_db("test").await.unwrap();
    assert_eq!(
        AlienVisitsPlanet::define_table().to_raw().build(),
        "DEFINE TABLE visits;"
    );

    assert_eq!(
        AlienVisitsPlanet::define_fields()
            .iter()
            .map(|x| x.to_raw().build())
            .collect::<Vec<_>>()
            .join("\n"),
        "DEFINE FIELD id ON TABLE visits TYPE record<visits>;
DEFINE FIELD in ON TABLE visits TYPE record<any>;
DEFINE FIELD out ON TABLE visits TYPE record<any>;
DEFINE FIELD timeVisited ON TABLE visits TYPE duration;
DEFINE FIELD age ON TABLE visits TYPE int;
DEFINE FIELD created ON TABLE visits TYPE datetime;
DEFINE FIELD lifeExpectancy ON TABLE visits TYPE duration;
DEFINE FIELD linePolygon ON TABLE visits TYPE geometry<feature>;
DEFINE FIELD territoryArea ON TABLE visits TYPE geometry<feature>;
DEFINE FIELD home ON TABLE visits TYPE geometry<feature>;
DEFINE FIELD tags ON TABLE visits TYPE array<string>;
DEFINE FIELD weapon ON TABLE visits TYPE record<weapon>;
DEFINE FIELD spaceShips ON TABLE visits TYPE array<record<space_ship>>;"
    );

    Ok(())
}

#[tokio::test]
async fn test_node_atttributes_explicit() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    assert_eq!(
        AlienVisitsPlanetExplicit::define_table().to_raw().build(),
        "DEFINE TABLE visits_explicit;"
    );

    assert_eq!(
        AlienVisitsPlanetExplicit::define_fields()
            .iter()
            .map(|x| x.to_raw().build())
            .collect::<Vec<_>>()
            .join("\n"),
        "DEFINE FIELD id ON TABLE visits_explicit TYPE record<visits_explicit>;
DEFINE FIELD in ON TABLE visits_explicit TYPE record<any>;
DEFINE FIELD out ON TABLE visits_explicit TYPE record<any>;
DEFINE FIELD timeVisited ON TABLE visits_explicit TYPE duration;"
    );

    Ok(())
}

#[tokio::test]
async fn test_node_type_atttribute_explicit() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    assert_eq!(
        AlienVisitsPlanetWithExplicitAttributes::define_table()
            .to_raw()
            .build(),
        "DEFINE TABLE visits_with_explicit_attributes;"
    );

    assert_eq!(
        AlienVisitsPlanetWithExplicitAttributes::define_fields()
            .iter()
            .map(|x| x.to_raw().build())
            .collect::<Vec<_>>()
            .join("\n"),
        "DEFINE FIELD id ON TABLE visits_with_explicit_attributes TYPE record<visits_with_explicit_attributes>;
DEFINE FIELD in ON TABLE visits_with_explicit_attributes TYPE record<any>;
DEFINE FIELD out ON TABLE visits_with_explicit_attributes TYPE record<any>;
DEFINE FIELD name ON TABLE visits_with_explicit_attributes TYPE string;
DEFINE FIELD age ON TABLE visits_with_explicit_attributes TYPE int;
DEFINE FIELD created ON TABLE visits_with_explicit_attributes TYPE datetime;
DEFINE FIELD lifeExpectancy ON TABLE visits_with_explicit_attributes TYPE duration;
DEFINE FIELD territoryArea ON TABLE visits_with_explicit_attributes TYPE geometry<polygon>;
DEFINE FIELD home ON TABLE visits_with_explicit_attributes TYPE geometry<point>;
DEFINE FIELD tags ON TABLE visits_with_explicit_attributes TYPE array<string>;
DEFINE FIELD weapon ON TABLE visits_with_explicit_attributes TYPE record<weapon>;
DEFINE FIELD spaceShips ON TABLE visits_with_explicit_attributes TYPE array<record<space_ship>>;"
    );

    Ok(())
}
