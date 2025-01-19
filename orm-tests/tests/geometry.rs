#![allow(clippy::type_complexity)]

/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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
use std::time::Duration;
use surreal_orm::Buildable;
use surreal_orm::{
    statements::{insert, select},
    All, Model, Node, Object, Operatable, ReturnableSelect, ReturnableStandard, SchemaGetter,
    SurrealId, SurrealSimpleId, ToRaw,
};
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

use geo::Coord;

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    name: String,
}

#[allow(clippy::type_complexity)]
#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "company")]
struct Company {
    id: SurrealId<Self, i32>,
    name: String,
    founded: chrono::DateTime<chrono::Utc>,

    #[orm(nest_array = Person)]
    founders: Vec<Person>,

    // #[orm(nest_array = "Person", ty = "array<array<object>>")]
    #[orm(nest_array = Person)]
    founders_multiple_nesting: Vec<Vec<Person>>,

    #[orm(nest_array = Person)]
    founders_10: Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Person>>>>>>>>>>,

    tags: Vec<String>,
    home: geo::Point,
}

#[allow(clippy::type_complexity)]
#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = company_2_for_testing)]
struct Company2ForTesting {
    id: SurrealId<Self, i32>,
    name: String,
    founded: chrono::DateTime<chrono::Utc>,

    #[orm(nest_array = "Person")]
    founder1: Person,

    #[orm(nest_array = "Person")]
    // founders: Vec<Person>,
    founders: [Person; 3],

    // #[orm(nest_array = "Person", ty = "array<array<object>>")]
    #[orm(nest_array = "Person")]
    founders_multiple_nesting: Vec<[Person; 28]>,

    #[orm(nest_array = "Person")]
    founders_10: Vec<Vec<Vec<Vec<Option<Vec<Vec<Vec<Vec<Vec<Person>>>>>>>>>>,

    tags: Vec<String>,
    home: geo::Point,
}

#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "gen_z_company")]
struct GenZCompany {
    id: SurrealSimpleId<Self>,
    name: String,
    founded: chrono::DateTime<chrono::Utc>,
    #[orm(nest_array = "Person")]
    founders: Vec<Person>,
    tags: Vec<String>,
    home: geo::Point,
}

#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "test_point")]
struct TestPoint {
    id: SurrealId<Self, i32>,
    home_point: geo::Point,
}

#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "test_linestring")]
struct TestLinestring {
    id: SurrealId<Self, i32>,
    home_linestring: geo::LineString,
}

#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "test_polygon")]
struct TestPolygon {
    id: SurrealId<Self, i32>,
    home_polygon: geo::Polygon,
}

#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "test_multilinestring")]
struct TestMultilinestring {
    id: SurrealId<Self, i32>,
    home_multilinestring: geo::MultiLineString,
}

#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "test_multipoint")]
struct TestMultipoint {
    id: SurrealId<Self, i32>,
    home_multipoint: geo::MultiPoint,
}

#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "test_multipolygon")]
struct TestMultipolygon {
    id: SurrealId<Self, i32>,
    home_multipolygon: geo::MultiPolygon,
}

#[derive(Node, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "test_geometrycollection")]
pub struct TestGeometrycollection {
    id: SurrealId<Self, i32>,
    home_geometrycollection: Vec<GeometryCollection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeometryCollection(geo::Geometry);

impl From<GeometryCollection> for surrealdb::sql::Geometry {
    fn from(value: GeometryCollection) -> Self {
        match value.0 {
            geo::Geometry::Point(p) => p.into(),
            geo::Geometry::LineString(ls) => ls.into(),
            geo::Geometry::Polygon(p) => p.into(),
            geo::Geometry::MultiPoint(mp) => mp.into(),
            geo::Geometry::MultiLineString(mls) => mls.into(),
            geo::Geometry::MultiPolygon(mp) => mp.into(),
            _ => unreachable!(),
        }
    }
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = "book")]
pub struct Book {
    id: SurrealSimpleId<Self>,
    title: String,
    content: String,
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
    let geometry_collection = vec![
        GeometryCollection(geo::Geometry::Point(point)),
        GeometryCollection(geo::Geometry::LineString(linestring)),
    ];

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
            founded: chrono::DateTime::from_naive_utc_and_offset(
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
            founders_multiple_nesting: vec![
                vec![
                    Person {
                        name: "John Doe".to_string(),
                    },
                    Person {
                        name: "Jane Doe".to_string(),
                    },
                ],
                vec![
                    Person {
                        name: "John Doe".to_string(),
                    },
                    Person {
                        name: "Jane Doe".to_string(),
                    },
                ],
            ],
            founders_10: vec![],
            tags: vec!["foo".to_string(), "bar".to_string()],
            home: (45.3, 78.1).into(),
        },
        Company {
            id: Company::create_id(2),
            name: "Apple Inc.".to_string(),
            founded: chrono::DateTime::from_naive_utc_and_offset(
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
            founders_multiple_nesting: vec![],
            founders_10: vec![],
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
            founded: chrono::DateTime::from_naive_utc_and_offset(
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
            founders_multiple_nesting: vec![],
            founders_10: vec![],
            tags: vec!["foo".to_string(), "bar".to_string()],
            home: (45.3, 78.1).into(),
        },
        Company {
            id: Company::create_id(2),
            name: "Apple Inc.".to_string(),
            founded: chrono::DateTime::from_naive_utc_and_offset(
                chrono::NaiveDate::from_ymd_opt(1974, 5, 3)
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
            founders_multiple_nesting: vec![],
            founders_10: vec![],
            tags: vec!["foo".to_string(), "bar".to_string()],
            home: (63.0, 21.0).into(),
        },
    ];

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;

    // Insert companies
    insert(companies).return_many(db.clone()).await.unwrap();

    let c = Company::schema();
    let select_query = &select(All)
        .from(Company::get_table())
        .where_(c.tags.any_like("foo"))
        .timeout(Duration::from_secs(20))
        .parallel();

    let selected_original = select_query
        .return_many::<Company>(db.clone())
        .await
        .unwrap();
    insta::assert_debug_snapshot!(selected_original);

    insert::<GenZCompany>(select_query)
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

#[test]
fn test_company_field_definitions() {
    let company_defs = Company::define_fields()
        .iter()
        .map(|f| f.build())
        .collect::<Vec<String>>()
        .join("\n");

    insta::assert_snapshot!(company_defs);
}
