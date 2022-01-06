use anyhow::Result;
use tonic::{Request, Response, Status};
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

pub struct GreeterApp;

impl GreeterApp {
    pub fn get_server() -> GreeterServer<MyGreeter> {
        let greeter = MyGreeter::default();

        GreeterServer::new(greeter)
    }
}
