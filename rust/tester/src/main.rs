use std::{collections::BTreeMap, time::Duration};

use chrono::{DateTime, Utc};
use poem::{error::InternalServerError, session::SessionStorage, web::Json, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb_rs::{embedded, embedded::Db, Surreal};

#[derive(Serialize, Deserialize, Debug, Default)]
struct Session {
    // #[serde(skip_serializing)]
    // session_id: Option<String>,
    expires_at: Option<DateTime<Utc>>,
    session_data: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Surreal::connect::<surrealdb_rs::storage::Mem>(())
        .await
        .unwrap();

    db.use_ns("namespace").use_db("database").await.unwrap();
    let xx: Option<BTreeMap<String, Value>> = db
        .update(("poem", "session_id"))
        .content(Session {
            // session_id: Some(session_id.to_string()),
            // session_id: Some("session_id".to_string()),
            expires_at: Some(Utc::now().checked_add_days(chrono::Days::new(1)).unwrap()),
            // expires: expires.map(|expires| Utc::now() + expires),
            session_data: json!({"key": "lowo".to_string()}),
        })
        .await
        .unwrap();
    println!("Hello, world!: {xx:?}");

    let all: Vec<Session> = db.select("poem").await.unwrap();
    println!("Sez result!: {all:?}");

    let mm: Session = db.select(("poem", "session_id")).await.unwrap();
    println!("Sez result!: {mm:?}");

    // let selected: Session = db.select("poem:session_id").await.unwrap();
    let selected: Session = db
        .query("select * from poem:session_id")
        .await
        .unwrap()
        .get(0, 0)
        .unwrap();
    println!("select result!: {selected:?}");

    // let deleted = db.delete(("poem", "session_id")).await.unwrap();
    let sx = "session_id";
    // let cleanup: Option<()> = db.query("delete from type::table($tb) where expires < $expires").bind("tb", "poem:session_id").await.unwrap().get(0,0).unwrap();
    // println!("select result!: {cleanup:?}");
    // let deleted: Option<()> = db.query("delete FROM type::table($tb) where expires < time::now()").bind("tb", "poem").await.unwrap().get(0,0).unwrap();
    // let cleanup: Option<()> = db
    //     .query("delete FROM type::thing($tb, $id) where expires < type::datetime($expires_at)")
    //     .bind("tb", "poem")
    //     .bind("id", "session_id")
    //     .bind("expires_at", Utc::now().checked_add_days(chrono::Days::new(1)).unwrap())
    //     .await
    //     .unwrap()
    //     .get(0, 0)
    //     .unwrap();

          let session: Option<BTreeMap<String, Value>>= 
            db
            .query("SELECT * FROM type::thing($tb, $id) WHERE expires_at IS NULL OR expires_at > $expires_at")
            .bind("tb", "poem")
            .bind("id", "session_id")
            .bind("expires_at", Utc::now())
            .await
            .unwrap()
            .get(0, 0).unwrap();
    println!("sx result!: {session:?}");

    let afer_delete: Option<Session> = db.select(("poem", "session_id")).await.unwrap();
    println!("ager delete result!: {afer_delete:?}");

    // println!("xxxx:{}", Utc::now());
    Ok(())
}
