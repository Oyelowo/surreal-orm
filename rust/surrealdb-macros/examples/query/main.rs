use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use surrealdb::engine::local::Mem;
use surrealdb::opt::auth::Root;
use surrealdb::sql::thing;
use surrealdb::sql::Datetime;
use surrealdb::Surreal;

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
use surrealdb_macros::query_insert;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Company {
    name: String,
    founded: Datetime,
    founders: Vec<Person>,
    tags: Vec<String>,
}

async fn test_it() -> surrealdb::Result<()> {
    // async fn test_it() {
    let companies = vec![
        Company {
            name: "Acme Inc.".to_string(),
            // founded: "1967-05-03".to_string(),
            founded: Datetime::default(),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
        },
        Company {
            name: "Apple Inc.".to_string(),
            // founded: "1967-05-03".to_string(),
            founded: Datetime::default(),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
        },
    ];
    let xx = Company {
        name: "Acme Inc.".to_string(),
        // founded: "1967-05-03".to_string(),
        founded: Datetime::default(),
        founders: vec![
            Person {
                name: "John Doe".to_string(),
            },
            Person {
                name: "Jane Doe".to_string(),
            },
        ],
        tags: vec!["foo".to_string(), "bar".to_string()],
    };

    let mm = query_insert::InsertStatement::new("company".into())
        .insert_all(companies)
        .build()
        .unwrap();

    let db = Surreal::new::<Mem>(()).await.unwrap();

    db.use_ns("test").use_db("test").await?;
    // let mut results = db.query(mm.0).bind(("company", user.clone())).await?;
    // let results = Arc::new(db.query(mm.0));
    // let results = db.query(mm.0);
    let results = mm.1.iter().fold(db.query(mm.0), |acc, val| {
        // res.
        let results = acc.bind(val);
        results
    });
    // for b in mm.1 {
    //     let results = results.bind(b);
    //     // let mut results = db.query(mm.0).bind(("company", user.clone())).await?;
    // }
    //
    let mut results = results.await.unwrap();

    // print the created user:
    let user: Option<User> = results.take(0).unwrap();
    println!("userQueryyy result: {user:?}");
    // println!("xrearXXXXX ---- = {}", serde_json::to_string(&mm).unwrap());
    Ok(())
}
#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // let db = Surreal::new::<File>("localhost:8001").await?;
    // // let db = Surreal::new::<File>("lowona").await?;
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
    // let users = vec![user.clone(), user.clone()];
    // let users_str = users
    //     .iter()
    //     .map(|u| serde_json::to_string(&u).unwrap())
    //     .collect::<Vec<_>>();
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
    // let mut response = response.await?;
    // // print all users:
    // let users: Vec<User> = response.take(0)?;
    // println!("user: {users:?}");
    //
    // println!("==========================================");
    // println!("==========================================");

    test_it().await.unwrap();
    Ok(())
}
