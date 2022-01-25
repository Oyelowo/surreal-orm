use crate::post::Post;
// use chrono::{self, TimeZone};
// use mongodb::bson::doc;
use wither::{self, prelude::Migrating};

impl Migrating for Post {
    // Define any migrations which your model needs in this method.
    // As this is an interval migration, it will deactivate itself after the given threshold
    // date, so you could leave it in your code for as long as you would like.
    fn migrations() -> Vec<Box<dyn wither::Migration<Post>>> {
        vec![
            // Box::new(wither::IntervalMigration {
            //     name: "copy-authorId-to_-uthorIds-field".to_string(),
            //     // NOTE: use a logical time here. A day after your deployment date, or the like.
            //     threshold: chrono::Utc.ymd(2022, 4, 1).and_hms(1, 0, 0),
            //     filter: doc! {"authorIds": doc!{"$exists": true}},
            //     set: Some(doc! {"authorIds": ""}),
            //     unset: Some(doc! {"authorId": ""}),
            // }),
        ]
    }
}
