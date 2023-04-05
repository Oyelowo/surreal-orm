/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use geo::LineString;
use geo::MultiLineString;
use geo::MultiPoint;
use geo::MultiPolygon;
use geo::Point;
use geo::Polygon;

use geo::point;
use geo::polygon;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::engine::local::Mem;
use surrealdb::opt::RecordId;
use surrealdb::sql;
use surrealdb::sql::Datetime;
// use surrealdb::sql::Geometry;
use surrealdb::sql::Uuid;
use surrealdb::Surreal;
use surrealdb_orm::Buildable;
use surrealdb_orm::RunnableSelect;
use surrealdb_orm::ToRaw;
// use surrealdb_derive::SurrealdbNode;
use std::time::Duration;

use geo::Coord;
use surrealdb::sql::statements::CommitStatement;
use surrealdb_orm::{
    statements::{insert, select},
    All, Geometry, Operatable, Parametric, Runnable, RunnableDefault, RunnableStandard, SurrealId,
    SurrealdbNode,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    name: String,
}
#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "company")]
struct Company {
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[builder(default, setter(strip_option))]
    id: Option<SurrealId>,
    nam: Uuid,
    name: String,
    founded: Datetime,
    founders: Vec<Person>,
    tags: Vec<String>,
    home: sql::Geometry,
}

#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "gen_z_company")]
struct GenZCompany {
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[builder(default, setter(strip_option))]
    id: Option<SurrealId>,
    nam: Uuid,
    name: String,
    founded: Datetime,
    founders: Vec<Person>,
    tags: Vec<String>,
    home: sql::Geometry,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "book")]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<SurrealId>,
    title: String,
    content: String,
}

fn create_test_company(geom: impl Into<sql::Geometry>) -> Company {
    let company = Company {
        id: Some(RecordId::from(("company", "lowo")).into()),
        nam: Uuid::try_from("285cfebe-a7f2-4100-aeb3-7f73998fff02").unwrap(),
        name: "Mana Inc.".to_string(),
        founded: "1967-05-03".into(),
        founders: vec![
            Person {
                name: "John Doe".to_string(),
            },
            Person {
                name: "Jane Doe".to_string(),
            },
        ],
        tags: vec!["foo".to_string(), "bar".to_string()],
        home: geom.into(),
    };
    company
}

async fn create_geom_test(geom: impl Into<sql::Geometry>) -> surrealdb::Result<String> {
    let company = create_test_company(geom);
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;

    // let results = insert::<Company>(company);
    let results = insert(company).return_one(db).await.unwrap().unwrap();

    Ok(serde_json::to_string(&results).unwrap())
}

#[tokio::test]
async fn point() -> surrealdb::Result<()> {
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    // let company = create_geom_test(point).await?;
    // insta::assert_snapshot!(company);
    println!("sql::Geom {:?}", sql::Geometry::Point(point.into()));
    println!("Geom basic{:?}", point);
    println!("Geom own{:?}", Geometry(point.into()));
    // insta::assert_debug_snapshot!(sql::Geometry::Point(point.into()));
    Ok(())
}

#[tokio::test]
async fn linestring() -> surrealdb::Result<()> {
    let ls = LineString(vec![
        Coord {
            x: -122.33583,
            y: 47.60621,
        },
        Coord {
            x: -122.33583,
            y: 47.60622,
        },
        Coord {
            x: -122.33584,
            y: 47.60622,
        },
        Coord {
            x: -122.33584,
            y: 47.60621,
        },
        Coord {
            x: -122.33583,
            y: 47.60621,
        },
    ]);

    let company = create_geom_test(ls).await?;
    insta::assert_snapshot!(company);
    Ok(())
}

#[tokio::test]
async fn polygon() -> surrealdb::Result<()> {
    let polygon = polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        // (x: 0.0, y: 0.0),
        // (x: 4.0, y: 0.0),
        // (x: 4.0, y: 1.0),
        // (x: 1.0, y: 1.0),
        // (x: 1.0, y: 4.0),
        // (x: 0.0, y: 4.0),
        // (x: 0.0, y: 0.0),
    ];
    //
    let company = create_geom_test(polygon).await?;
    insta::assert_snapshot!(company);

    let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );

    let company_complex = create_geom_test(poly).await?;
    println!(
        "ZMZMZMZM {}",
        serde_json::to_string(&company_complex).unwrap()
    );
    insta::assert_snapshot!(company_complex);
    Ok(())
}

#[tokio::test]
async fn multipoint() -> surrealdb::Result<()> {
    let points = MultiPoint(vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0),
        (2.0, 35.0).into(),
    ]);

    let company = create_geom_test(points).await?;
    insta::assert_snapshot!(company);
    Ok(())
}

#[tokio::test]
async fn multiline() -> surrealdb::Result<()> {
    let linestring1 = LineString(vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 1.0 },
        Coord { x: 2.0, y: 2.0 },
    ]);
    let linestring2 = LineString(vec![
        Coord { x: 3.0, y: 3.0 },
        Coord { x: 4.0, y: 4.0 },
        Coord { x: 5.0, y: 5.0 },
    ]);
    let multiline_string = MultiLineString(vec![linestring1, linestring2]);

    let company = create_geom_test(multiline_string).await?;
    insta::assert_snapshot!(company);
    Ok(())
}

#[tokio::test]
async fn multipolygon() -> surrealdb::Result<()> {
    let polygon1 = Polygon::new(
        LineString(vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 2.0, y: 2.0 },
            Coord { x: 0.0, y: 0.0 },
        ]),
        vec![],
    );
    let polygon2 = Polygon::new(
        LineString(vec![
            Coord { x: 3.0, y: 3.0 },
            Coord { x: 4.0, y: 4.0 },
            Coord { x: 5.0, y: 5.0 },
            Coord { x: 3.0, y: 3.0 },
        ]),
        vec![],
    );
    let poly3 = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.43),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
    let multi_polygon = MultiPolygon(vec![polygon1, polygon2, poly3]);
    insta::assert_snapshot!(serde_json::to_string(&multi_polygon).unwrap());
    let company = create_geom_test(multi_polygon).await?;
    insta::assert_snapshot!(company);
    Ok(())
}

#[tokio::test]
async fn geom_collection() -> surrealdb::Result<()> {
    let point = Point(Coord { x: 0.0, y: 0.0 });
    let linestring = LineString(vec![Coord { x: 1.0, y: 1.0 }, Coord { x: 2.0, y: 2.0 }]);
    let geometry_collection = vec![sql::Geometry::Point(point), sql::Geometry::Line(linestring)];
    let geometry_collection = sql::Geometry::Collection(geometry_collection);
    let company = create_geom_test(geometry_collection).await?;
    insta::assert_snapshot!(company);
    Ok(())
}

#[tokio::test]
async fn insert_many() -> surrealdb::Result<()> {
    let companies = vec![
        Company {
            id: Some("company:1".try_into().unwrap()),
            name: "Acme Inc.".to_string(),
            founded: "1967-05-03".into(),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
            nam: Uuid::try_from("725cfebe-a7f2-4100-aeb3-7f73998fff02").unwrap(),
            home: (45.3, 78.1).into(),
        },
        Company {
            id: Some("company:2".try_into().unwrap()),
            name: "Apple Inc.".to_string(),
            founded: "1967-05-03".into(),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
            nam: Uuid::try_from("375cfebe-a7f2-4100-aeb3-7f73998fff02").unwrap(),
            home: (63.0, 21.0).into(),
        },
    ];

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;

    let results = insert(companies).return_many(db).await.unwrap();

    insta::assert_debug_snapshot!(results);
    Ok(())
}

#[tokio::test]
async fn insert_from_select_query() -> surrealdb::Result<()> {
    let companies = vec![
        Company {
            id: Some("company:1".try_into().unwrap()),
            name: "Acme Inc.".to_string(),
            founded: "1967-05-03".into(),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
            nam: Uuid::try_from("725cfebe-a7f2-4100-aeb3-7f73998fff02").unwrap(),
            home: (45.3, 78.1).into(),
        },
        Company {
            id: Some("company:2".try_into().unwrap()),
            name: "Apple Inc.".to_string(),
            founded: "1967-05-03".into(),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
            nam: Uuid::try_from("375cfebe-a7f2-4100-aeb3-7f73998fff02").unwrap(),
            home: (63.0, 21.0).into(),
        },
    ];

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;
    // Insert companies
    // let results = insert(companies).return_many(db.clone()).await.unwrap();
    let results = insert(companies).return_many(db.clone()).await.unwrap();
    // results.into_iter().collect();

    // let results = insert(companies)
    //     .return_many(db.clone())
    //     // .return_many_test::<Vec<_>>(db.clone())
    //     .await
    //     .unwrap();

    println!("QQQQQQINS {:?}", results);
    // db.clone()
    //     .query(format!("{}", CommitStatement))
    //     .await
    //     .unwrap();
    //
    let c = Company::schema();
    let select_query = select(All)
        .from(&SurrealId::try_from("company:2").unwrap())
        .where_(c.tags.any_like("foo"))
        .parallel();
    // .return_one(db.clone())
    // .await
    // .unwrap();

    dbg!(select_query.clone().get_bindings());
    dbg!(select_query.clone().to_raw());
    assert_eq!(select_query.clone().build().to_string(), "dfd");
    // println!("BindSel {:?}", select_query.get_bindings());
    let one_ = select_query
        .return_one::<Company>(db.clone())
        .await
        .unwrap();
    println!("SSSSSSS {:?}", one_);

    println!(
        "SSSSSSS {:?}",
        select_query
            .return_many::<Company>(db.clone())
            .await
            .unwrap()
    );

    let ref select_query = select(All)
        .from(Company::get_table_name())
        .where_(c.tags.any_like("foo"))
        .timeout(Duration::from_secs(20))
        .parallel();

    println!("BindSel {:?}", select_query.get_bindings());
    println!(
        "SSSSSSS {:?}",
        select_query
            .return_many::<Company>(db.clone())
            .await
            .unwrap()
    );
    // TODO: The fall back to return_one if list returned not working. Investigate.
    // let results = insert::<GenZCompany>(select_query)
    //     .return_one(db.clone())
    //     .await
    //     .unwrap();
    let results: Vec<GenZCompany> = insert(select_query).return_many(db.clone()).await.unwrap();

    let results = insert::<GenZCompany>(select_query)
        .return_many(db.clone())
        .await
        .unwrap();

    let results: Vec<GenZCompany> = insert(select_query).return_many(db.clone()).await.unwrap();
    // let results = insert::<GenZCompany>(select_query)
    //     .return_many(db.clone())
    //     // .return_many_explicit::<Vec<GenZCompany>>(db.clone())
    //     .await
    //     .unwrap();

    insta::assert_debug_snapshot!(results);
    Ok(())
}

