mod app_analytics;
mod configs;
mod greetings;
mod music;

use anyhow::Result;
use app_analytics::AnalyticsApp;
use configs::Configs;
use greetings::GreeterApp;
use music::MusicFanApp;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let Configs { application, .. } = Configs::init();
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
