use anyhow::Result;
use hello_world::{greeter_client::GreeterClient, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}
pub mod music {
    tonic::include_proto!("music");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
