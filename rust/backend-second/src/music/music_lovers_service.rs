use anyhow::{Context, Result};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    //#[path = "grpc_proto.hello_world.rs"]
    //pub mod hello_world;
    tonic::include_proto!("music_lovers"); // name of proto package
}

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::lovers_server::{Lovers, LoversServer};
use hello_world::{HelloReply, HelloRequest};


#[derive(Debug, Default)]
pub struct MyLover {}

#[tonic::async_trait]
impl Lovers for MyLover {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}
