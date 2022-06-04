use anyhow::Result;
use grpc_mongo::{
    app::{app_analytics::AnalyticsApp, greetings::GreeterApp, music::MusicFanApp},
    utils::configuration,
};
use tonic::transport::Server;

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
