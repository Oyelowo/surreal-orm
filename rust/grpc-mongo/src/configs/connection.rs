use mongodb::{Client, Database};

use super::Configs;

pub async fn establish_connection() -> Database {
    let Configs { database, .. } = Configs::init();

    Client::with_uri_str(database.get_url())
        .await
        .expect("failed to get database connection")
        .database(database.name.as_str())
}
