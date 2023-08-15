/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{AlienVisitsPlanet, AlienVisitsPlanetExplicit};
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
        "DEFINE FIELD id ON TABLE visits TYPE record (visits);
DEFINE FIELD in ON TABLE visits TYPE record();
DEFINE FIELD out ON TABLE visits TYPE record();
DEFINE FIELD timeVisited ON TABLE visits TYPE duration;"
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
        "DEFINE FIELD id ON TABLE visits_explicit TYPE record (visits_explicit);
DEFINE FIELD in ON TABLE visits_explicit TYPE record();
DEFINE FIELD out ON TABLE visits_explicit TYPE record();
DEFINE FIELD timeVisited ON TABLE visits_explicit TYPE duration;"
    );

    Ok(())
}
