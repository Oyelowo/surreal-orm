mod app;
mod utils;

use anyhow::Result;
use app::{app_analytics::AnalyticsApp, greetings::GreeterApp, music::MusicFanApp};
use tonic::transport::Server;
use utils::configuration;

#[tokio::main]
async fn main() -> Result<()> {
    let application = configuration::get_app_config();
    let addr = application.get_url();

    Server::builder()
        .accept_http1(true)
        .add_service(GreeterApp::get_server())
        .add_service(MusicFanApp::get_server())
        .add_service(AnalyticsApp::get_server())
        .serve(addr)
        .await?;

    Ok(())
}
