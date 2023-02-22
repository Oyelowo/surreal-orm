use std::ops::Deref;

use geo::{Geometry, LineString};
use geojson::GeoJson;
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql;

#[derive(Debug, Serialize, Deserialize)]
struct Dayo(geo::Geometry);

impl Deref for Dayo {
    type Target = geo::Geometry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Lowo {
    home: geo::Geometry,
}

fn main() {
    let json = r#"{
    "type": "LineString",
    "coordinates": [[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]
}"#;

    // let geo_json: serde_json::Value = serde_json::from_str(json).unwrap();
    let geo_json: sql::Geometry = serde_json::from_str(json).unwrap();
    println!("Deserialized GeoJSON: {:?}", geo_json);

    // let coordinates: Vec<[f64; 2]> =
    //     serde_json::from_value(geo_json["coordinates"].clone()).unwrap();
    // // let line_string = LineString(coordinates);
    //
    // let geo_type: String = serde_json::from_value(geo_json["type"].clone()).unwrap();
    // let geometry = match geo_type.as_str() {
    //     "LineString" => Geometry::new(line_string.into()),
    //     _ => unimplemented!(),
    // };

    // println!("Deserialized GeoJSON: {:?}", geometry);
}
