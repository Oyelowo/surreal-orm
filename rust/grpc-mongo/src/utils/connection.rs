use std::process;

use common::configurations::mongodb::MongodbConfigs;
use mongodb::Database;

pub async fn establish_connection() -> Database {
    let database = MongodbConfigs::get();

    database.get_database().unwrap_or_else(|e| {
        log::error!("failed to get mongo database. Error: {e}");
        process::exit(-1)
    })
}
