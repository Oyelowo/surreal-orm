mod greetings;

use anyhow::Result;
use greetings::{GreeterApp, MusicLoverApp};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .accept_http1(true)
        .add_service(GreeterApp::new())
        .add_service(MusicLoverApp::new())
        .serve(addr)
        .await?;


    Ok(())
}
