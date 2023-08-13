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
use surrealdb::sql;
use surrealdb::sql::Datetime;
use surrealdb_orm::SurrealdbCrudNode;
use std::time::Duration;
use surrealdb::sql::Uuid;
use surrealdb::Surreal;
use surrealdb_orm::{
    statements::{insert, select},
    All, Geometry, Operatable, Parametric, ReturnableSelect, ReturnableStandard, SchemaGetter,
    SurrealId, SurrealSimpleId, SurrealdbModel, SurrealdbNode, SurrealdbObject, ToRaw,
};

use geo::Coord;

#[derive(SurrealdbObject, Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    name: String,
}
#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "company")]
struct Company {
    id: SurrealId<Self, i32>,
    name: String,
    founded: chrono::DateTime<chrono::Utc>,
    founders: Vec<Person>,
    tags: Vec<String>,
    home: geo::Point,
}

#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "gen_z_company")]
struct GenZCompany {
    id: SurrealSimpleId<Self>,
    name: String,
    founded: chrono::DateTime<chrono::Utc>,
    founders: Vec<Person>,
    tags: Vec<String>,
    home: geo::Point,
}


#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "test_point")]
struct TestPoint {
    id: SurrealId<Self, i32>,
    home_point: geo::Point,
}

#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "test_linestring")]
struct TestLinestring {
    id: SurrealId<Self, i32>,
    home_linestring: geo::LineString,
}

#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "test_polygon")]
struct TestPolygon {
    id: SurrealId<Self, i32>,
    home_polygon: geo::Polygon,
}

#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "test_multilinestring")]
struct TestMultilinestring {
    id: SurrealId<Self, i32>,
    home_multilinestring: geo::MultiLineString,
}

#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "test_multipoint")]
struct TestMultipoint {
    id: SurrealId<Self, i32>,
    home_multipoint: geo::MultiPoint,
}

#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "test_multipolygon")]
struct TestMultipolygon {
    id: SurrealId<Self, i32>,
    home_multipolygon: geo::MultiPolygon,
}

#[derive(SurrealdbNode, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "test_geometrycollection")]
struct TestGeometrycollection {
    id: SurrealId<Self, i32>,
    home_geometrycollection: Vec<geo::Geometry>,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "book")]
pub struct Book {
    id: SurrealSimpleId<Self>,
    title: String,
    content: String,
}

fn create_test_company(geom: impl Into<geo::Point>) -> Company {
    let company = Company {
        id: Company::create_id(32),
        name: "Mana Inc.".to_string(),
        founded: chrono::DateTime::from_utc(
            chrono::NaiveDate::from_ymd_opt(1967, 5, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            chrono::Utc,
        ),
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
        // home: geo::point!(x: 40.02f64, y: 116.34),
    };
    company
}

async fn create_geom_test(geom: impl Into<geo::Point>) -> surrealdb::Result<String> {
    let company = create_test_company(geom);
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;

    // let results = company.create().get_one(db).await.unwrap();
    let results = insert(company).return_one(db).await.unwrap().unwrap();

    Ok(serde_json::to_string(&results).unwrap())
}


macro_rules! create_test_data_assertion {
    ($test_data: expr) => {
        let geom = $test_data;
        insta::assert_debug_snapshot!(geom);
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await?;

        let results = insert(geom).return_one(db).await;

        let res_string = serde_json::to_string(&results.unwrap()).unwrap();
        insta::assert_snapshot!(res_string);
    };
}

#[tokio::test]
async fn polygon_with_exterior_interior() -> surrealdb::Result<()> {
    let polygon = polygon!(
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

    create_test_data_assertion!(TestPolygon {
        id: TestPolygon::create_id(32),
        home_polygon: polygon,
    });
    Ok(())
}

#[tokio::test]
async fn multipoint() -> surrealdb::Result<()> {
    let points = MultiPoint(vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0),
        (2.0, 35.0).into(),
    ]);

    create_test_data_assertion!(TestMultipoint {
        id: TestMultipoint::create_id(32),
        home_multipoint: points,
    });
    Ok(())
}

#[tokio::test]
async fn point() -> surrealdb::Result<()> {
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    create_test_data_assertion!(TestPoint {
        id: TestPoint::create_id(32),
        home_point: point,
    });
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

    create_test_data_assertion!(TestLinestring {
        id: TestLinestring::create_id(32),
        home_linestring: ls,
    });
    Ok(())
}

#[tokio::test]
async fn polygon() -> surrealdb::Result<()> {
    let polygon = polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        (x: 0.0, y: 0.0),
        (x: 4.0, y: 0.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 0.0, y: 4.0),
        (x: 0.0, y: 0.0),
    ];

    create_test_data_assertion!(TestPolygon {
        id: TestPolygon::create_id(32),
        home_polygon: polygon,
    });
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

    create_test_data_assertion!(TestMultilinestring {
        id: TestMultilinestring::create_id(32),
        home_multilinestring: multiline_string,
    });
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
    create_test_data_assertion!(TestMultipolygon {
        id: TestMultipolygon::create_id(32),
        home_multipolygon: multi_polygon,
    });
    Ok(())
}

#[tokio::test]
async fn geom_collection() -> surrealdb::Result<()> {
    let point = Point(Coord { x: 0.0, y: 0.0 });
    let linestring = LineString(vec![Coord { x: 1.0, y: 1.0 }, Coord { x: 2.0, y: 2.0 }]);
    let geometry_collection = vec![geo::Geometry::Point(point), geo::Geometry::LineString(linestring)];
    
    create_test_data_assertion!(TestGeometrycollection {
        id: TestGeometrycollection::create_id(32),
        home_geometrycollection: geometry_collection,
    });
    Ok(())
}

#[tokio::test]
async fn insert_many() -> surrealdb::Result<()> {
    let companies = vec![
        Company {
            id: Company::create_id(32),
            name: "Acme Inc.".to_string(),
            founded: chrono::DateTime::from_utc(
                chrono::NaiveDate::from_ymd_opt(1967, 5, 3)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                chrono::Utc
            ),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
            home: (45.3, 78.1).into(),
        },
        Company {
            id: Company::create_id(2),
            name: "Apple Inc.".to_string(),
            founded: chrono::DateTime::from_utc(
                chrono::NaiveDate::from_ymd_opt(1967, 5, 3)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                chrono::Utc
            ),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
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
            id: Company::create_id(1),
            name: "Acme Inc.".to_string(),
            founded: chrono::DateTime::from_utc(
                chrono::NaiveDate::from_ymd_opt(1967, 5, 3)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                chrono::Utc
            ),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
            home: (45.3, 78.1).into(),
        },
        Company {
            id: Company::create_id(2),
            name: "Apple Inc.".to_string(),
            founded: chrono::DateTime::from_utc(
                chrono::NaiveDate::from_ymd_opt(1974, 5, 3)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                chrono::Utc
            ),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
            home: (63.0, 21.0).into(),
        },
    ];

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;
    
    // Insert companies
    let results = insert(companies).return_many(db.clone()).await.unwrap();

    let c = Company::schema();
    let ref select_query = select(All)
        .from(Company::get_table_name())
        .where_(c.tags.any_like("foo"))
        .timeout(Duration::from_secs(20))
        .parallel();


    let selected_original  = select_query
            .return_many::<Company>(db.clone())
            .await
            .unwrap();
    insta::assert_debug_snapshot!(selected_original);

    let results = insert::<GenZCompany>(select_query)
        .return_one(db.clone())
        .await
        .expect_err("Too many items returned. Therefore, you should not use return_one method since there are multiple entries");
    
    let results: Vec<GenZCompany> = insert(select_query).return_many(db.clone()).await.unwrap();
    insta::assert_debug_snapshot!(results);

    let results = insert::<GenZCompany>(select_query)
        .return_many(db.clone())
        .await
        .unwrap();
    insta::assert_debug_snapshot!(results);

    let results: Vec<GenZCompany> = insert(select_query).return_many(db.clone()).await.unwrap();
    insta::assert_debug_snapshot!(results);
    Ok(())
}
