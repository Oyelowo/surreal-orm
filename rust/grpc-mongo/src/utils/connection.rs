use mongodb::{Client, Database};

use super::configuration::get_db_config;

pub async fn establish_connection() -> Database {
    let database = get_db_config();

    Client::with_uri_str(database.get_url())
        .await
        .expect("failed to get database connection")
        .database(database.name.as_str())
}
