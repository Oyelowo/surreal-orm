

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = MySqlConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Mysql");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = MySqlPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Mysql.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}