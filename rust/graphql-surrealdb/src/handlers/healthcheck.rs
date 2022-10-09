use anyhow::Context;
use poem::web::Data;
use poem::{error::Result, handler};
use poem::{IntoResponse, Response};
use reqwest::StatusCode;
use surrealdb::Datastore;

pub async fn get_redis_connection(
    redis: Data<&redis::Client>,
) -> anyhow::Result<redis::aio::Connection> {
    redis
        .get_async_connection()
        .await
        .context("Failed to get redis connection")
}

#[handler]
pub async fn healthz(
    db: Data<&Datastore>,
    redis: Data<&redis::Client>,
) -> Result<String> {
// ) -> Result<impl IntoResponse> {
    // let mut connection = get_redis_connection(redis).await?;
    // let redis = redis::cmd("PING")
    //     .query_async::<_, ()>(&mut connection)
    //     .await;

    // // // Ping the server to see if you can connect to the cluster
    // // let surrealdb = db.run_command(doc! {"ping": 1}, None).await;

    // let auth_url_data = match (db) {
    //     (Ok(_), Ok(_)) => {
    //         log::info!("Connected successfully.");
    //         Response::builder().status(StatusCode::OK).body("ok")
    //     }
    //     _ => Response::builder()
    //         .status(StatusCode::INTERNAL_SERVER_ERROR)
    //         .body("Dependencies not ready"),
    // };
    // Ok(auth_url_data)
    todo!()
}

#[handler]
pub async fn liveness() -> impl IntoResponse {
    Response::builder().status(StatusCode::OK).body("Ok")
}
