#![allow(clippy::derive_partial_eq_without_eq)]
use std::{net::SocketAddr, process};

use anyhow::Result;
use common::configurations::application::ApplicationConfigs;
use grpc_mongo::app::{app_analytics::AnalyticsApp, greetings::GreeterApp, music::MusicFanApp};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let application = ApplicationConfigs::default();
    let addr: SocketAddr = application.get_url().parse().unwrap_or_else(|e| {
        log::error!("Failed to parse application url to socket address. Error: {e}");
        process::exit(-1)
    });

    Server::builder()
        .accept_http1(true)
        .add_service(GreeterApp::get_server())
        .add_service(MusicFanApp::get_server())
        .add_service(AnalyticsApp::get_server())
        .serve(addr)
        .await?;

    Ok(())
}
