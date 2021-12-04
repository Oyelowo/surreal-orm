use anyhow::{Result};
use tonic::{transport::Server};
use backend_second::{get_greeter_server, GreeterApp};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse()?;


    Server::builder()
    .accept_http1(true)
        .add_service(GreeterApp::new())
        .serve(addr)
        .await?;

    Ok(())
}
