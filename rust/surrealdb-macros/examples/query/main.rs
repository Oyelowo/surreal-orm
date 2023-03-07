use std::ops::Deref;
use std::sync::Arc;

use geo::coord;
use geo::Coord;
use geo::Coordinate;
use geo::GeodesicIntermediate;
use geo::GeometryCollection;
use geo::Line;
use geo::LineString;
use geo::MultiLineString;
use geo::MultiPoint;
use geo::MultiPolygon;
use geo::Point;
use geo::Polygon;

use geo::line_string;
use geo::point;
use geo::polygon;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use surrealdb::engine::local::Mem;
use surrealdb::opt::auth::Root;
use surrealdb::opt::RecordId;
use surrealdb::sql;
use surrealdb::sql::thing;
use surrealdb::sql::Datetime;
use surrealdb::sql::Geometry;
use surrealdb::sql::Limit;
use surrealdb::sql::Uuid;
use surrealdb::Surreal;

// use geo
// use geo::point;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
struct User {
    id: String,
    name: String,
    company: String,
    founded: Datetime,
}

use serde_json::Result;
use serde_json::{Map, Value};
use surrealdb_derive::SurrealdbNode;
use surrealdb_macros::db_field::cond;
use surrealdb_macros::db_field::Parametric;
use surrealdb_macros::query_insert;
use surrealdb_macros::query_insert::updater;
use surrealdb_macros::query_insert::Updater;
use surrealdb_macros::query_select;
use surrealdb_macros::query_select::select;
use surrealdb_macros::query_select::All;
use surrealdb_macros::value_type_wrappers::GeometryCustom;
use surrealdb_macros::value_type_wrappers::SurrealId;
use surrealdb_macros::DbField;
use surrealdb_macros::SurrealdbModel;
use surrealdb_macros::SurrealdbNode;
fn mana() {
    #[derive(Serialize, Deserialize)]
    struct Country {
        title: String,
        continent: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
        countries: Vec<Country>,
    }

    #[derive(Serialize, Deserialize)]
    struct Address {
        street: String,
        city: String,
        countries: Vec<Country>,
        owner: Person,
    }

    fn print_an_address() -> Result<String> {
        // Some data structure.

        let address = Address {
            street: "10 Downing Street".to_owned(),
            city: "London".to_owned(),
            countries: vec![
                Country {
                    title: "Canada".into(),
                    continent: "NA".into(),
                },
                Country {
                    title: "finland".into(),
                    continent: "EU".into(),
                },
            ],
            owner: Person {
                name: "Oyelowo".into(),
                age: 90,
                countries: vec![
                    Country {
                        title: "Canada".into(),
                        continent: "NA".into(),
                    },
                    Country {
                        title: "finland".into(),
                        continent: "EU".into(),
                    },
                ],
            },
        };

        //
        // insert.values(address);
        //
        // insert(Company {
        //     name: "SurrealDB".into(),
        //     founded: Date(2021-09-10)
        // })
        // [("name", String("SurrealDB")), ("founded", Date(2021-09-10))]
        //
        // INSERT INTO company (name, founded) VALUES ($name, $founded);
        //
        //
        // INSERT INTO company (name, founded) VALUES ('SurrealDB', '2021-09-10');
        //
        // INSERT INTO company (name, founded) VALUES ($name, $founded);
        // bindings: [
        //           (name, 'SurrealDB')
        //           (founded, '2021-09-10')
        // ]
        //
        // INSERT INTO company (name, founded) VALUES ($arg1, $arg2);
        // bindings: [
        //           (arg1, 'SurrealDB')
        //           (arg2, '2021-09-10')
        // ]
        //
        // [("street", String("10 Downing Street")), ("city", String("London")), ("countries", Array [Object {"title": String("Canada"), "continent": String("NA")}, Object {"title": String("finland"), "continent": String("E
        // U")}]), ("owner", Object {"name": String("Oyelowo"), "age": Number(90), "countries": Array [Object {"title": String("Canada"), "continent": String("NA")}, Object {"title": String("finland"), "continent": String("
        // EU")}]})]                                                                                                                                      git:(204-surrealdb-orm-implement-fully-compliant-insert-query|✚2⚑20
        //
        //
        //
        //
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&address)?;

        // Print, write to a file, or send to an HTTP server.

        // println!("ADDRES = {}", j);

        Ok(j)
    }
    // print_an_address().unwrap();

    // fn dmain() {
    let json = r#"{"key1": "value1", "key2": 42}"#;

    let result = json_to_vec(print_an_address().unwrap().as_str());

    println!("{:?}", result);
}

fn json_to_vec(json: &str) -> Vec<(String, Value)> {
    let parsed: Map<String, Value> = serde_json::from_str(json).unwrap();

    parsed
        .into_iter()
        .map(|(key, value)| (key, value))
        .collect()
}

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
    // id: String,
    // nam: Uuid,
    name: String,
    // founded: Datetime,
    // founders: Vec<Person>,
    // tags: Vec<String>,
    // location: Geometry,
    home: GeometryCustom,
}

// {
//     type: "Point",
//     coordinates: [-0.118092, 51.509865],
// }
//
//
//
// journey
// UPDATE city:london SET centre = {
//     type: "Point",
//     coordinates: [-0.118092, 51.509865],
// };
//
#[derive(Debug, Serialize, Deserialize, Clone)]
struct City {
    name: String,
    centre: Geometry,
}
async fn test_it() -> surrealdb::Result<()> {
    // async fn test_it() {
    let a = Line::new(coord! { x: 0., y: 0. }, coord! { x: 1., y: 1. });
    let b = Line::new(coord! { x: 0., y: 0. }, coord! { x: 1.001, y: 1. });
    //
    // println!(
    //     "OPOOOOO____{}",
    //     serde_json::to_value(&companies.clone()).unwrap()
    // );
    let a = Line::new(coord! { x: 0., y: 0. }, coord! { x: 1., y: 1. });

    let a = LineString::from(a);
    let b = LineString::from(b);
    // let a = MultiPoint::from(a);
    // let a: MultiPoint<_> = vec![(0., 0.), (1., 2.)].into();
    // let a: MultiLineString<_> = vec![(0., 0.), (1., 2.)].into();
    // let a: MultiLineString<_> = vec![a, b].into();

    let a = polygon![
        (x: 0.0, y: 0.0),
        (x: 4.0, y: 0.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 0.0, y: 4.0),
        (x: 0.0, y: 0.0),
    ];
    let a = point! {
        x: 40.02f64,
        y: 116.34,
    };
    // let a = (33f64, 44f64);
    let a = Point::new(0.0, 0.0); // create a point at the origin
    let a = Point::from((45.0, 90.0)); // cr
                                       //
                                       //
                                       // Coordinate(Coord{})
    let a = LineString(vec![
        Coordinate::from((0.0, 0.0)),
        Coordinate::from((1.0, 1.0)),
        Coordinate::from((2.0, 0.0)),
    ]);
    let a = LineString(vec![
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
    let points = vec![
        Coord { x: 0.0, y: 0.0 },
        Coord { x: 1.0, y: 1.0 },
        Coord { x: 2.0, y: 2.0 },
    ];
    // let multipoint = MultiPoint(points);
    // let a = MultiPoint(vec![Point::new(0.0, 0.0), Point::new(1.0, 1.0)]);
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
    let a = MultiLineString(vec![linestring1, linestring2]);
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
    let a = MultiPolygon(vec![polygon1, polygon2]);
    let point = Point(Coordinate { x: 0.0, y: 0.0 });
    let linestring = LineString(vec![
        Coordinate { x: 1.0, y: 1.0 },
        Coordinate { x: 2.0, y: 2.0 },
    ]);
    let geometry_collection = GeometryCollection(vec![
        geo::Geometry::Point(point),
        geo::Geometry::LineString(linestring),
    ]);
    let a = Geometry::from(a);

    let companies = vec![
        Company {
            id: None,
            // id: "company:1".into(),
            name: "Acme Inc.".to_string(),
            // founded: "1967-05-03".to_string(),
            // founded: Datetime::default(),
            //
            // founders: vec![
            //     Person {
            //         name: "John Doe".to_string(),
            //     },
            //     Person {
            //         name: "Jane Doe".to_string(),
            //     },
            // ],
            // tags: vec!["foo".to_string(), "bar".to_string()],
            // nam: Uuid::new(), // location: Geometry::Point((45.0, 45.0).into()),
            // location: (45.0, 45.0).into(),
            home: GeometryCustom(a.clone().into()),
            // home: GeometryCustom((45.0, 45.0).into()),
            // home: LineString(vec![Coord { x: 34.6, y: 34.6 }]),
        },
        Company {
            id: None,
            // id: "company:2".into(),
            name: "Apple Inc.".to_string(),
            // founded: "1967-05-03".to_string(),
            // founded: Datetime::default(),
            // founders: vec![
            //     Person {
            //         name: "John Doe".to_string(),
            //     },
            //     Person {
            //         name: "Jane Doe".to_string(),
            //     },
            // ],
            // tags: vec!["foo".to_string(), "bar".to_string()],
            // nam: Uuid::new(),
            // home: Point::new(25.3, 39.4).into(),
            // home: Geometry::Line(b.into()),
            home: GeometryCustom((63.0, 21.0).into()),
            // location: Geometry::Point((45.0, 45.0).into()),
        },
    ];
    // let a = Line::new(coord! { x: 0., y: 0. }, coord! { x: 1., y: 1. });
    // let b = Line::new(coord! { x: 0., y: 0. }, coord! { x: 1.001, y: 1. });
    let xx = Company {
        id: Some(RecordId::from(("company", "lowo")).into()),
        // id: "company:1".into(),
        name: "Mana Inc.".to_string(),
        // founded: "1967-05-03".to_string(),
        // founded: Datetime::default(),
        // founders: vec![
        //     Person {
        //         name: "John Doe".to_string(),
        //     },
        //     Person {
        //         name: "Jane Doe".to_string(),
        //     },
        // ],
        // tags: vec!["foo".to_string(), "bar".to_string()],
        // nam: Uuid::new(), // location: (63.0, 21.0).into(),
        // home: Geome(Geometry::Point((45.0, 45.0).into())),
        home: GeometryCustom(a.clone().into()),
        // home: Geome((63.0, 21.0).into()),
        // home: Geometry::Point(Point::new(20.2, 60.9)),
        // home: (63.0, 21.0).into(),
    };

    // let loc = serde_json::to_string(&xx.home).unwrap();
    println!("companyMMMMMM: {}", serde_json::to_string(&xx).unwrap());
    let loca = r#"{"type":"Point","coordinates":[65.0,21.0]}"#;
    let json = r#"{
    "type": "LineString",
    "coordinates": [[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]
}"#;
    let val = &serde_json::to_string(&a).unwrap();
    println!("VAAAL {val}");
    // println!("LOPERER{xx:#?}");

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;

    // let results = query_insert::InsertStatement::new()
    //     .insert(xx.clone())
    //     .insert_many(companies)
    //     // .get_one(db.clone())
    //     .get_many(db)
    //     .await
    //     .unwrap();

    // println!("==========================================");
    // println!("==========================================");
    // println!(
    //     "userQueryyy result: {}",
    //     serde_json::to_string(&results).unwrap()
    // );
    // // let mut results = results.await?;
    // println!("==========================================");
    // println!("==========================================");
    // // println!("userQueryyyAwaited result: {:#?}", results);
    // println!("==========================================");
    // println!("Value==========================================");
    // let user: Vec<Company> = results.take(0)?;
    // let user: Vec<Company> = user;
    // println!("nama result: {}", serde_json::to_string(&user).unwrap());
    // println!("xrearXXXXX ---- = {}", serde_json::to_string(&mm).unwrap());
    // println!("ERERERErere  mm1 = {:#?}", mm.clone().0);
    // println!("ERERERErere  mm2 = {:#?}", mm.clone().1);
    // println!(
    //     "ERERERErere  mm3 = {:}",
    //     serde_json::to_string(&mm.1).unwrap()
    // );

    // let mut results = db.query(mm.0).bind(("company", user.clone())).await?;
    // let results = Arc::new(db.query(mm.0));
    // let results = db.query(mm.0);
    // let results =
    //     mm.1.clone()
    //         .iter()
    //         .fold(db.query(mm.clone().0), |acc, val| {
    //             // res.
    //             let results = acc.bind(val);
    //             results
    //         });
    // let mut response = db.query("INSERT INTO mana { name: 'lowo'};");
    // let mut response = response.await?;
    // print all users:

    // let mut response = db.query("SELECT * fROM mana;");
    // let mut response = response.await?;

    // #[derive(Debug, Serialize, Deserialize, Clone)]
    // struct Mana {
    //     name: String,
    // }
    // let users: Option<Mana> = response.take(0)?;
    // println!("SAMAAAAAAAAA: {users:?}");
    // for b in mm.1 {
    //     let results = results.bind(b);
    //     // let mut results = db.query(mm.0).bind(("company", user.clone())).await?;
    // }
    //
    //     let mut results = db.query("
    // INSERT INTO company (name, founded) VALUES ('Acme Inc.', '1967-05-03'), ('Apple Inc.', '1976-04-01');
    // ");
    // let mut results = db
    //     .query(
    //         "
    // INSERT INTO company (id, name, home) VALUES (type::thing($tb, $id), $name, $home);
    // ",
    //     )
    //     .bind(("tb", "company"))
    //     .bind(("id", xx.id))
    //     // .bind(("home", xx.home))
    //     // .bind(("company", xx))
    //     .bind(("name", xx.name));
    // println!("==========================================");
    // println!("==========================================");
    // // println!("userQueryyy result: {:#?}", results);
    // let mut results = results.await?;
    // // println!("==========================================");
    // // println!("==========================================");
    // // println!("userQueryyyAwaited result: {:#?}", results);
    // // println!("==========================================");
    // // println!("Value==========================================");
    // // // println!(
    // // //     "ompany: {}",
    // // //     serde_json::to_string(&RecordId::from(("lowo", "kolo"))).unwrap()
    // // // );
    // let user: Option<Value> = results.take(0).expect("shit");
    //
    // println!("company: {}", serde_json::to_string(&user).unwrap());

    // // let user: Vec<Company> = user;
    // println!("nama result: {}", serde_json::to_string(&user).unwrap());
    // println!("xrearXXXXX ---- = {}", serde_json::to_string(&mm).unwrap());

    // UPDATE city:london SET centre = {
    //     type: "Point",
    //     coordinates: [-0.118092, 51.509865],
    // };
    // println!("2222==========================================");
    // let mut response = db.query("CREATE city:london SET name ='lowocity', centre = { type: \"Point\", coordinates: [-0.118092, 51.509865], };");
    // let mut response = db.query("CREATE city:london CONTENT $city;").bind((
    //     "city",
    //     City {
    //         name: "mars".into(),
    //         centre: (44.4, 27.1).into(),
    //     },
    // ));
    // println!("cityQueryyy result: {:#?}", response);
    // let mut response = response.await?;
    //
    // // let city: Option<City> = response.take(0)?;
    // let city: Option<Value> = response.take(0)?;
    //
    // println!("City: {}", serde_json::to_string(&city).unwrap());
    // println!("2222==========================================");
    // let mut response = db.query("SELECT * FROM company");
    // let mut response = response.await?;
    // // print all users:
    //
    // let users: Vec<Company> = response.take(0)?;
    // println!("company: {users:?}");
    // print the created user:
    Ok(())
}
#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // let db = Surreal::new::<File>("lowona").await?;
    // let db = Surreal::new::<Mem>(()).await.unwrap();
    //
    // // db.use_ns("namespace").use_db("database").await?;
    // db.use_ns("test").use_db("test").await?;
    //
    // // type::thing($tb, $id)
    // let sql = "CREATE user SET name = $name, company = $company";
    // let sql = "CREATE $id SET name = $name, company = $company, founded = $founded";
    // let sql = "CREATE user CONTENT $1";
    //
    // let sql = "INSERT INTO company $company";
    // // INSERT INTO company   //
    // // println!("Dfdfe {}", Datetime::default());
    // // println!("thingthinghting {}", thing("user:owo").unwrap().to_string());
    //
    // let user = User {
    //     // id: thing("user:owo").unwrap().to_string(),
    //     id: "user:owo".to_string(),
    //     // id: "john".to_owned(),
    //     name: "John Doe".to_owned(),
    //     company: "ACME Corporation".to_owned(),
    //     founded: Datetime::default(),
    // };
    //
    // // let users = vec![user.clone(), user.clone()];
    // // let users_str = users
    // //     .iter()
    // //     .map(|u| serde_json::to_string(&u).unwrap())
    // //     .collect::<Vec<_>>();
    // // println!("ushoud:  {:?}", users_str);
    // // println!("ushoud:  {:?}", json_to_vec(users_str.unwrap().as_str()));
    //
    // let mut results = db.query(sql).bind(("company", user.clone())).await?;
    //
    // // print the created user:
    // let user: Option<User> = results.take(0)?;
    // println!("userQuery result: {user:?}");
    //
    // let mut response = db
    //     .query("SELECT * FROM user WHERE name ~ $name")
    //     .bind(("name", "John"));
    //
    // let mut response = response.await?;
    // // print all users:
    // let users: Vec<User> = response.take(0)?;
    // println!("user: {users:?}");

    println!("==========================================");
    // println!("==========================================");

    // test_it().await.unwrap();
    let point = geo::point! {
        x: 40.02f64,
        y: 116.34,
    };

    let poly = polygon!(
            exterior: [
                (x: -111.35, y: 45.),
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
            ],);
    let a = updater("e").plus_equal(sql::Geometry::Point(point));
    println!("a = {a}");

    let a = updater("e").plus_equal(sql::Geometry::Polygon(poly.clone()));
    let a = updater("e").plus_equal(sql::Geometry::Polygon(poly));
    println!("a = {a}");

    let a = updater("e").increment_by(sql::Number::from(5));
    println!("a = {a}");

    let a = updater("e").increment_by(34);
    println!("a = {a}");

    let a = updater("e").decrement_by(923.54);
    println!("a = {a}");
    updater("e").increment_by("crm");
    DbField::new("d".to_string()).equal(5);
    // sql::Value::Param(sql::Param("trt"));

    let xx = sql::Field::Alone("dd".into());
    let xx = sql::Strand("dd".into());

    // sql::Param::from("jij".into());
    println!("xxx {} erere", &xx.0);

    let xx = sql::Value::Strand("Rer".into());
    let xx = sql::Table("Rer".into());
    let xx = DbField::new("oejfiner");
    // sql::Field;
    let cc: sql::Table = xx.into();
    println!("xxx {} erere", cc);
    // println!("xxx {}", serde_json::to_string(&cc).unwrap());
    // println!(
    //     "qqqq {}",
    //     sql::json(&serde_json::to_string(&cc.to_string()).unwrap()).unwrap()
    // );

    let company::Company { id, name, home, .. } = Company::schema();
    println!("xxxxvv {}", name.clone());
    let ref age = DbField::new("age");
    let firstName = &name;
    let lastName = &name;
    let line = line_string![
        (x: -21.95156, y: 64.1446),
        (x: -21.951, y: 64.14479),
        (x: -21.95044, y: 64.14527),
        (x: -21.951445, y: 64.145508),
    ];
    let polygon = polygon![
        (x: 0.9, y: 0.0),
        (x: 4.0, y: 0.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 0.0, y: 4.0),
        (x: 0.0, y: 0.0),
    ];
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
    let x = vec![1, 2, 3];
    let y = 9;
    let z = x.into_iter().chain(std::iter::once(y)).collect::<Vec<_>>();
    println!("{:?}", z); // prints [1, 2, 3, 9]
                         // let polygon: sql::Value = sql::Geometry::Polygon(polygon.into()).into();
                         // let ref mut query = queryb.select_all().from(Company::get_table_name()).where_(
                         //     cond(age.greater_than(id).greater_than(age))
                         //         .or(firstName.greater_than(90))
                         //         .or(firstName.greater_than(90))
                         //         .or(firstName.greater_than(90))
                         //         .or(firstName.greater_than(439))
                         //         .and(age.greater_than(150))
                         //         .and(age.greater_than(316))
                         //         .and(age.greater_than(711).greater_than_or_equal(421).equal(25))
                         //         .or(age.greater_than(382).greater_than_or_equal(975).equal(52)),
                         // );
    let ref mut query = select(All).from(Company::get_table_name()).where_(
        cond(age.greater_than(id.clone()))
            .or(firstName.like("Oyelowo"))
            .and(lastName.exactly_equal("Oyedayo"))
            .and(age.less_than(150))
            .or(age.greater_than(382).greater_than_or_equal(975).equal(52)),
    );
    println!("MAWAOOOO----->{}", query);
    println!("BINDAAAA----->{:?}", query.get_bindings());
    let xx = cond(age.greater_than(id.clone()));
    println!("AEKERJERJ----->{}", xx);
    println!("AEKERJERJ----->{:?}", xx.get_bindings());
    // let x = Student::schema()
    //     .writes__(empty())
    //     .book(id.equal(RecordId::from(("book", "blaze"))))
    //     .title;
    // println!("XXX----->{:?}", x);
    // println!("MANA----->{:?}", x.get_bindings());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple() {
        let sql = "(51.509865, -0.118092)";
        let res = geometry(sql);
        assert!(res.is_ok());
        let out = res.unwrap().1;
        assert_eq!("(51.509865, -0.118092)", format!("{}", out));
    }
}
