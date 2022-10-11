Posted on 24 Apr 2020 • Updated on 25 Oct 2020

Rust gRPC A beginners guide to gRPC with Rust
=============================================

[#rust](/t/rust) [#grpc](/t/grpc) [#tutorial](/t/tutorial)

[](#table-of-contents)Table of Contents
=======================================

1.  Introduction
2.  Protocol Buffer
3.  Rust and gRPC
4.  Creating a Server
5.  Creating a Client
6.  Streaming in gRPC
7.  Authentication
8.  Conclusion

[](#introduction)Introduction
=============================

HTTP and JSON is a very popular method for creating web APIs. HTTP and JSON make sense because it uses a very popular protocol. HTTP and JSON are text-based protocol which causes a performance problem since serializing JSON is a slow process, most HTTP and JSON or Rest APIs do not support streaming which means we cannot start processing the data before it arrives. Rest APIs have very good tooling and community support. Almost every programming language has a high-quality implementation for HTTP and serializing JSON. Rest architecture doesn’t fit every use case, it is difficult to provide client library for Rest APIs for every language and maintain these libraries. Since there is no language-independent method for defining the structure of JSON and HTTP requests, therefore, it is difficult to generate client libraries. gRPC is an attempt to tackle these problems.

[](#brief-intro-to-grpc)Brief Intro to gRPC
-------------------------------------------

gRPC is an open-source remote procedure call system developed by Google. gRPC allows the system to communicate in and out of data centers, efficiently transferring data from mobile, IoT devices, backends to one and other. gRPC came with plug able support for load balancing, authentication, tracing, etc. gRPC supports bidirectional streaming over HTTP/2. gRPC provides an idiomatic implementation in 10 languages. gRPC can generate efficient client libraries and uses the protocol buffer format for transferring data over the wire. Protocol buffers are a binary format for data transmission. Since protocol buffers are a binary protocol, it can be serialized fast and the structure of each message must be predefined.

[](#a-little-about-rust)A Little about Rust
-------------------------------------------

Rust is a systems programming language. Rust provides high level ergonomic with low-level control. Rust provides control over memory management without the hassle associated with it. Rust has good support for Asynchronous operation making it a good fit for writing networking applications. Rust has zero cost abstraction making it blazing fast.

[![Work Flow Source(https://blog.logrocket.com/creating-a-crud-api-with-node-express-and-grpc/)](https://res.cloudinary.com/practicaldev/image/fetch/s--2qhr7Str--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587676359774_grpc-2.png)](https://res.cloudinary.com/practicaldev/image/fetch/s--2qhr7Str--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587676359774_grpc-2.png)

[](#protocol-buffers)Protocol Buffers
=====================================

Protocol buffers are extensible, language-neutral data serializing mechanism. It is fast, small, and simple. Protocol buffers have a predefined structure with its syntax for defining messages and services. Services are functions that can be executed and messages are arguments passed to the function and values returned by these functions. There are two versions of protocol buffers. This tutorial would use version 3.

[](#syntax)Syntax
-----------------

Protocol Buffers have a very simple syntax. There are two things to be defined in a protocol buffer.

*   `service` It defines all the functionality that can be called on a particular service or server.
*   `message` It defines arguments and returns values of an `RPC` call.

The `syntax` and `package` must be defined in every protocol buffer file. Protocol buffer files are saved with `.proto` extension.  

        // version of protocol buffer used
        syntax = "proto3";
    
        // package name for the buffer will be used later
        package hello;
    
        // service which can be executed
        service Say {
        // function which can be called
          rpc Send (SayRequest) returns (SayResponse);
        }
    
        // argument
        message SayRequest {
        // data type and position of data
          string name = 1;
        }
    
        // return value
        message SayResponse {
        // data type and position of data
          string message = 1;
        }
    

Enter fullscreen mode Exit fullscreen mode

A service is defined by using `service` keyword then defining call using `rpc` keyword, `send` is the name of the call, it can be used to make a call, `SayRequest` define the argument `send` call takes and `SayResponse` defines the value returned by the call. Any number of the call can be defined in service.  

        service Say {
        // function which can be called
          rpc Send (SayRequest) returns (SayResponse);
          rpc Take (SayRequest) returns (SayResponse);
        }
    

Enter fullscreen mode Exit fullscreen mode

_Implementation of these function is not defined in the_ `*.proto*` _file, these implementations are provided by the server_

**Assign Number to Fields**  
The number assigned to a field is very important because it is used to recognize the field in binary data. It takes 1 byte to encode 0-15 numbers and 2 bytes for encoding 16-2047, it is wise to use 0-15 for frequently occurring data. It is also recommended to reserve a few numbers so that, these reserved numbers can be used later if some changes are made to format.

**Different Data Types**

Prototype support many data types include string, int, float, boolean, etc. These types can be repeated using `repeated` field attributes.

Protocol buffer syntax is explained in great detail in [official documentation.](https://developers.google.com/protocol-buffers/docs/proto)

[](#rust-and-grpc)Rust and gRPC
===============================

Rust ecosystem has grown quite big with very good quality crates. `tonic` is very performant gRPC implementation for Rust. This tutorial uses `tonic` as the gRPC implementation and `tonic-build` for compiling `.proto` files to client libraries.  
Let us start by creating a new cargo project using `cargo init`. Now we need to create an add a few dependencies and build dependencies. These will help us with our server and client.  

        [package]
        name = "grpc"
        version = "0.1.0"
        authors = ["Anshul Goyal <anshulgoel151999@gmail.com>"]
        edition = "2018"
    
        [dependencies]
        prost = "0.6.1"
        tonic = {version="0.2.0",features = ["tls"]}
        tokio = {version="0.2.18",features = ["stream", "macros"]}
        futures = "0.3"
    
        [build-dependencies]
        tonic-build = "0.2.0"
    

Enter fullscreen mode Exit fullscreen mode

This should be a configuration for `Cargo.toml` file. `prost` provides basic types for gRPC, `tokio` provide asynchronous runtime and `futures` for handling asynchronous streams.

[](#compiling-protocol-buffers)Compiling Protocol Buffers
---------------------------------------------------------

We would use `build.rs` for compiling our `.proto` files and include then in binary. `tonic-build` crate provides a method `compile_protos` which take the path to `.ptoto` file and compile it to rust definitions. First, we create a folder in the root directory named `proto` it will contain all of out `.proto` files. We create a `say.proto` file in this directory. With our `Say` service shown in the above example.

We create a `build.rs` with the following code.  

        fn main()->Result<(),Box<dyn std::error::Error>>{
        // compiling protos using path on build time
           tonic_build::compile_protos("proto/say.proto")?;
           Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

The above code will compile `proto/say.proto` file and save it in an `OUT_DIR` and add an environment variable `OUT_DIR` which is available at build time so that we can use it later in our code. We can also provide different options for compiling the protocol buffers. Now your directory structure should look like this:  

        ├── build.rs
        ├── Cargo.lock
        ├── Cargo.toml
        ├── proto
        │   └── say.proto
        ├── src
            ├── main.rs
    

Enter fullscreen mode Exit fullscreen mode

Now we have compiled our `.proto` files we would use it in our code using `tonic` utility. We would create a module for our server and client. Let us name it `hell.rs` and we would add the following code.  

        // this would include code generated for package hello from .proto file
        tonic::include_proto!("hello");
    
    

Enter fullscreen mode Exit fullscreen mode

[](#creating-a-server)Creating a Server
=======================================

Now we have compiled the protocol buffers we are ready to build our server. We have to provide the implementation for every service and `rpc` we defined. Service would be defined as traits and `rpc` would be a member function on these traits. Since Rust doesn't support async traits we have to use an `asyc_trait` macro for overcoming this limitation. We create a file named `server.rs` and add the following code.

`**tonic-build**` **would automatically compile** `**.proto**` **following rust naming conventions and best practices.**  

        use tonic::{transport::Server, Request, Response, Status};
        use hello::say_server::{Say, SayServer};
        use hello::{SayResponse, SayRequest};
        mod hello; 
    
        // defining a struct for our service
        #[derive(Default)]
        pub struct MySay {}
    
        // implementing rpc for service defined in .proto
        #[tonic::async_trait]
        impl Say for MySay {
        // our rpc impelemented as function
            async fn send(&self,request:Request<SayRequest>)->Result<Response<SayResponse>,Status>{
        // returning a response as SayResponse message as defined in .proto
                Ok(Response::new(SayResponse{
        // reading data from request which is awrapper around our SayRequest message defined in .proto
                     message:format!("hello {}",request.get_ref().name),
                }))
            }
        }
    
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // defining address for our service
            let addr = "[::1]:50051".parse().unwrap();
        // creating a service
            let say = MySay::default();
            println!("Server listening on {}", addr);
        // adding our service to our server.
            Server::builder()
                .add_service(SayServer::new(say))
                .serve(addr)
                .await?;
            Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

In Rust, messages are represented as structs and services as traits and RPC as functions. We `impl` the trait for our struct and pass it to our server. In our example, we would create a send function which takes `Request` as an argument which contains details about the request and wraps our message `SayRequest` which can be accessed using `.get_ref` method. Now let us run this by adding a bin block to our `Cargo.toml`.  

        [[bin]]
        name = "server"
        path = "src/server.rs"
    

Enter fullscreen mode Exit fullscreen mode

This will help us testing and maintaining code in save repo but it is not suggested for a large project.  
Now if we run this with command `cargo run` `--``bin server`. We can see our server running at `127:0:0:1:50051`.

[![Example](https://res.cloudinary.com/practicaldev/image/fetch/s--fqbtPeGX--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587642193283_image.png)](https://res.cloudinary.com/practicaldev/image/fetch/s--fqbtPeGX--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587642193283_image.png)

[](#creating-a-client)Creating a Client
=======================================

Our server is ready, now let's test it by creating a client. Since we have compiled our protocol buffer we can import our `hello.rs` file and use it. We create a `client.rs` file and add the following code.  

        use hello::say_client::SayClient;
        use hello::SayRequest;
    
        mod hello;
    
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // creating a channel ie connection to server
            let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
            .connect()
            .await?;
        // creating gRPC client from channel
            let mut client = SayClient::new(channel);
        // creating a new Request
            let request = tonic::Request::new(
                SayRequest {
                   name:String::from("anshul")
                },
            );
        // sending request and waiting for response
            let response = client.send(request).await?.into_inner();
            println!("RESPONSE={:?}", response);
            Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

We add a bin key to our `Cargo.toml`  

        [[bin]]
        name = "client"
        path = "src/client.rs"
    

Enter fullscreen mode Exit fullscreen mode

We create a channel that is an HTTP/2 connection that can be used then from our client. HTTP/2 support streams that can be used by gRPC. Now if we run our client with command `cargo run` `--bin client` we can see see the response.

[![](https://res.cloudinary.com/practicaldev/image/fetch/s--ViilGlG7--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587643536436_image.png)](https://res.cloudinary.com/practicaldev/image/fetch/s--ViilGlG7--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587643536436_image.png)

[](#error-handling)Error Handling
=================================

Error handling in gRPC is done using Status. `tonic` provide `Status` enum which can be returned in case of error with appropriate error message.  

        Err(Status::unauthenticated("Token not found"))
    

Enter fullscreen mode Exit fullscreen mode

gRPC support bare-bone error handling but you can extend yourself using protocol buffer here is [detail explanation](https://cloud.google.com/apis/design/errors#error_model)

[](#streaming-in-grpc)Streaming In gRPC
=======================================

HTTP/2 supports streaming and gRPC provides a nice interface for using it. We can start sending the response even before the client finish sends the request. We can use it to provide an efficient service. The server has not to wait for the request from the client to complete the request. We need to make a few changes to our protocol buffers so that it reflects that we support streaming. We need to make changes to our rust code also. Rust has quite good support for asynchronous I/O. We would `tokio` to stream response and request.

[](#streaming-response)Streaming Response
-----------------------------------------

We would start by the streaming server since most of the time server would be sending a large amount of data. We would use a queue for sending data by multiplexing different task on a single thread. `tokio` provide very excellent multi sender single receiver channel.

**Changes to protocol buffers**

We change the code of protocol buffer to the following. We use a stream keyword in `rpc` and specify that the `rpc` call will return a stream of messages `SayResponse`.  

        service Say {
        // function which can be called
          rpc Send (SayRequest) returns (SayResponse);
        // we specify that we return a stream
          rpc SendStream(SayRequest) returns (stream SayResponse);
        }
    

Enter fullscreen mode Exit fullscreen mode

**Changes to Server Code**  
We would use `tokio::sync::mpsc` for communicating between futures. We send multiple responses using this channel. We would use `tokio::spawn` to create a new task that can be then scheduled. We add the following code to our `server.rs` file.  

        use tokio::sync::mpsc;
        use tonic::{transport::Server, Request, Response, Status};
        use hello::say_server::{Say, SayServer};
        use hello::{SayRequest, SayResponse};
        mod hello;
    
        #[derive(Default)]
        pub struct MySay {}
        #[tonic::async_trait]
        impl Say for MySay {
        // Specify the output of rpc call
            type SendStreamStream=mpsc::Receiver<Result<SayResponse,Status>>;
        // implementation for rpc call
            async fn send_stream(
                &self,
                request: Request<SayRequest>,
            ) -> Result<Response<Self::SendStreamStream>, Status> {
        // creating a queue or channel
                let (mut tx, rx) = mpsc::channel(4);
        // creating a new task
                tokio::spawn(async move {
        // looping and sending our response using stream
                    for _ in 0..4{
        // sending response to our channel
                        tx.send(Ok(SayResponse {
                            message: format!("hello"),
                        }))
                        .await;
                    }
                });
        // returning our reciever so that tonic can listen on reciever and send the response to client
                Ok(Response::new(rx))
            }
            async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
                Ok(Response::new(SayResponse {
                    message: format!("hello {}", request.get_ref().name),
                }))
            }
        }
    
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let addr = "[::1]:50051".parse().unwrap();
            let say = MySay::default();
            println!("Server listening on {}", addr);
            Server::builder()
                .add_service(SayServer::new(say))
                .serve(addr)
                .await?;
            Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

We need to change the main function, we just add a new function to trait and a type to specify our output. In this new `send_stream` function we create a channel so that we can send a response and return the receiver. The receiver implements the`Stream` trait so it can be streamed by HTTP/2 and the sender can be used by multiple threads and it implements `Sink` trait. We have created a bounded channel but we can also use an unbounded channel.

**Changes in Client Code**  
We need to make changes to response handling. Since it would be a stream now, we would just listen to this stream and print the response. Streams help to write non-blocking code and use resources more efficiently.  

        use hello::say_client::SayClient;
        use hello::SayRequest;
        mod hello;
    
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
            .connect()
            .await?;
            let mut client = SayClient::new(channel);
            let request = tonic::Request::new(
                SayRequest {
                   name:String::from("anshul")
                },
            );
        // now the response is stream
            let mut response = client.send_stream(request).await?.into_inner();
        // listening to stream
            while let Some(res) = response.message().await? {
                println!("NOTE = {:?}", res);
            }
            Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

[](#streaming-request)Streaming Request
---------------------------------------

Sometimes all the data is not available, for example in a game all the data is not available then it would make stream the data and send all the data available and sending rest when available. This allows using data more efficiently on user devices. We need a few changes to our code.

**Changes to protocol buffer**  
We would use the `stream` keyword to specify the argument is a stream. We would use `stream` to specify that our `rpc` takes a stream as an argument.  

        // service which can be executed
        service Say {
        // function which can be called
          rpc Send (SayRequest) returns (SayResponse);
          rpc SendStream(SayRequest) returns (stream SayResponse);
        // taking a stream as response
          rpc ReceiveStream(stream SayRequest) returns (SayResponse);
        }
    

Enter fullscreen mode Exit fullscreen mode

**Changes to Server**  
We need our server to accept the stream as the request. We would listen on the stream and collect. Then we would respond when the stream finishes. It will save our resources since we can wait on stream asynchronously.  

        use tokio::sync::mpsc;
        use tonic::{transport::Server, Request, Response, Status};
        use hello::say_server::{Say, SayServer};
        use hello::{SayRequest, SayResponse};
        mod hello;
        #[derive(Default)]
        pub struct MySay {}
        #[tonic::async_trait]
        impl Say for MySay {
        // .. rest of rpcs
        // create a new rpc to receive a stream
            async fn receive_stream(
                &self,
                request: Request<tonic::Streaming<SayRequest>>,
            ) -> Result<Response<SayResponse>, Status> {
        // converting request into stream
                let mut stream = request.into_inner();
                let mut message = String::from("");
        // listening on stream
                while let Some(req) = stream.message().await? {
                    message.push_str(&format!("Hello {}\n", req.name))
                }
        // returning response
                Ok(Response::new(SayResponse { message }))
            }
        }
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let addr = "[::1]:50051".parse().unwrap();
            let say = MySay::default();
            println!("Server listening on {}", addr);
            Server::builder()
                .add_service(SayServer::new(say))
                .serve(addr)tes());
                .await?;
            Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

**Changes to Client**  
We would now program our client to send a stream to our server. For this, we would mimic a stream using `futures` crate and create a stream from a vector.  

        use futures::stream::iter;
        use hello::say_client::SayClient;
        use hello::SayRequest;
        mod hello;
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
                .connect()
                .await?;
            let mut client = SayClient::new(channel);
        // creating a stream
            let request = tonic::Request::new(iter(vec![
                SayRequest {
                    name: String::from("anshul"),
                },
                SayRequest {
                    name: String::from("rahul"),
                },
                SayRequest {
                    name: String::from("vijay"),
                },
            ]));
        // sending stream
            let response = client.receive_stream(request).await?.into_inner();
            println!("RESPONSE=\n{}", response.message);
            Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

[![](https://res.cloudinary.com/practicaldev/image/fetch/s--si_ESsIL--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587657121220_image.png)](https://res.cloudinary.com/practicaldev/image/fetch/s--si_ESsIL--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587657121220_image.png)

[](#bidirectional-stream)Bidirectional Stream
---------------------------------------------

The bidirectional stream is also supported by gRPC. The bidirectional stream is just a combination of streaming requests and streaming responses. Here is a quick example.

**Protocol buffer**  

    
        // version of protocol buffer used
        syntax = "proto3";
        // package name for buffer will be used later
        package hello;
        // service which can be executed
        service Say {
        // takes a stream and returns a stream
          rpc Bidirectional(stream SayRequest) returns (stream SayResponse);
        }
        // argument
        message SayRequest {
        // data type and position of data
          string name = 1;
        }
        // return value
        message SayResponse {
        // data type and position of data
          string message = 1;
        }
    

Enter fullscreen mode Exit fullscreen mode

**Sever**  

        use tokio::sync::mpsc;
        use tonic::{transport::Server, Request, Response, Status};
        use hello::say_server::{Say, SayServer};
        use hello::{SayRequest, SayResponse};
        mod hello;
        #[derive(Default)]
        pub struct MySay {}
        #[tonic::async_trait]
        impl Say for MySay {
        // defining return stream
            type BidirectionalStream = mpsc::Receiver<Result<SayResponse, Status>>;
            async fn bidirectional(
                &self,
                request: Request<tonic::Streaming<SayRequest>>,
            ) -> Result<Response<Self::BidirectionalStream>, Status> {
        // converting request in stream
                let mut streamer = request.into_inner();
        // creating queue
                let (mut tx, rx) = mpsc::channel(4);
                tokio::spawn(async move {
        // listening on request stream
                    while let Some(req) = streamer.message().await.unwrap(){
        // sending data as soon it is available
                        tx.send(Ok(SayResponse {
                            message: format!("hello {}", req.name),
                        }))
                        .await;
                    }
                });
        // returning stream as receiver
                Ok(Response::new(rx))
            }
        }
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let addr = "[::1]:50051".parse().unwrap();
            let say = MySay::default();
            println!("Server listening on {}", addr);
            Server::builder()
                .add_service(SayServer::new(say))
                .serve(addr)
                .await?;
            Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

**Client**  

        use futures::stream::iter;
        use hello::say_client::SayClient;
        use hello::SayRequest;
        mod hello;
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
                .connect()
                .await?;
            let mut client = SayClient::new(channel);
        // creating a client
            let request = tonic::Request::new(iter(vec![
                SayRequest {
                    name: String::from("anshul"),
                },
                SayRequest {
                    name: String::from("rahul"),
                },
                SayRequest {
                    name: String::from("vijay"),
                },
            ]));
        // calling rpc
            let mut response = client.bidirectional(request).await?.into_inner();
        // listening on the response stream
            while let Some(res) = response.message().await? {
                println!("NOTE = {:?}", res);
            }
            Ok(())
        }
    

Enter fullscreen mode Exit fullscreen mode

[![Example](https://res.cloudinary.com/practicaldev/image/fetch/s--bwobuM0K--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587658480089_image.png)](https://res.cloudinary.com/practicaldev/image/fetch/s--bwobuM0K--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://paper-attachments.dropbox.com/s_59BE9A5DB3EFBE4D28334D77611F2324BC09BD447D8DEAD39CC6417F47CAB169_1587658480089_image.png)

[](#authentication)Authentication
=================================

Authentication is a very important aspect of a system. gRPC comes with plug able authentication support. gRPC support mainly two types of authentication:

*   Token-based authentication
*   TLS based authentication ## Token-Based Authentication

In this tutorial, we would use JWT based authentication. JWT or JSON web token provides an open-source and stateless authentication mechanism. We would `jsonwebtoken` crate for creating JWT and validating it. We would just see how we can use JWT with gRPC.

**Server**  
We would need to add an interceptor, that would validate token, if the token is not valid, we would just close the request, if the token is valid then we forward the request to our handlers.  

        fn interceptor(req:Request<()>)->Result<Request<()>,Status>{
            let token=match req.metadata().get("authorization"){
                Some(token)=>token.to_str(),
                None=>return Err(Status::unauthenticated("Token not found"))
            };
            // do some validation with token here ...
            Ok(req)
        }
    

Enter fullscreen mode Exit fullscreen mode

If we return `Ok` then the request would be passed on to functions but if we return `Err` with status the request is closed with provided status. We create a service with this interceptor.  

        let say = MySay::default();
        let ser = SayServer::with_interceptor(say,interceptor);
        Server::builder().add_service(ser).serve(addr).await?;
    

Enter fullscreen mode Exit fullscreen mode

**Client**  
We need to add an interceptor to our client also. We would add it to our main function as a closure.  

            let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
                .connect()
                .await?;
            let token = get_token();// an method to get token can be a rpc call etc.
            let mut client = SayClient::with_interceptor(channel, move |mut req: Request<()>| {
        // adding token to request.
                req.metadata_mut().insert(
                    "authorization",
                    tonic::metadata::MetadataValue::from_str(&token).unwrap(),
                );
                Ok(req)
            });
    

Enter fullscreen mode Exit fullscreen mode

[](#mutual-tls-based-authentication)Mutual TLS Based Authentication
-------------------------------------------------------------------

TLS stands for Transport Layer Security, it is recommended by gRPC documentation to encrypt HTTP/2 connection with TLS. We would TLS to authenticate both client and server. This is called Mutual TLS. We would create a private key and public key for both client and server. We would also create a Certificate Authority certificate so that we can sign our TLS certificate. We would require OpenSSL for creating certificates.

**Creating Certificates**

OpenSSL is a command-line utility for creating keys and encryption-related stuff. We would start by creating a Certification Authority certificate.

    openssl genrsa -des3 -out my_ca.key 2048
    

Enter fullscreen mode Exit fullscreen mode

This would act as our signing key. We would use it to sign our TLS certificate. Next, we create our certificate which is called the root CA certificate. It is used to validate if our TLS certificate is  
validated or not.

    openssl req -x509 -new -nodes -key my_ca.key -sha256 -days 1825 -out my_ca.pem
    

Enter fullscreen mode Exit fullscreen mode

This command would ask you a few questions. Details enter in this doesn’t matter. If you can get this certificate on every device on earth you become a certificate signing authority like Let’s Encrypt etc. Now let's create our server key and certificate.

    openssL genrsa -out server.key 2048
    

Enter fullscreen mode Exit fullscreen mode

This command will generate a key for our server. Now we create a certificate signing request for our key.

    openssl req -new -sha256 -key server.key -out server.csr
    

Enter fullscreen mode Exit fullscreen mode

This would ask you some questions. Now we create a `server.ext` file. This file would contain our name, our domain, or subdomain.

    authorityKeyIdentifier=keyid,issuer
    basicConstraints=CA:FALSE
    keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
    subjectAltName = @alt_names
    
    [alt_names]
    DNS.1 = localhost
    

Enter fullscreen mode Exit fullscreen mode

We add our identity in the form of DNS. Now we run the following command

    openssl x509 -req -in server.csr -CA my_ca.pem -CAkey my_ca.key -CAcreateserial -out server.pem -days 1825 -sha256 -extfile server.ext
    

Enter fullscreen mode Exit fullscreen mode

You don’t need to provide your private key or server key. This might ask you a few questions and passcode provided when generating the Certificate Authority key.  
You can generate a Certificate for the client using the same certificate authority. Now we have all the required certificates to let configure our server and client.

[](#configuring-client-and-server)Configuring Client and Server
---------------------------------------------------------------

`tonic` support TLS using `rust-tls`. We can configure TLS by following the method.

This shows how to configure the client for TLS.  

            // getting certificate from disk
            let cert=include_str!("../client.pem");
            let key=include_str!("../client.key");
            // creating identify from key and certificate
            let id=tonic::transport::Identity::from_pem(cert.as_bytes(),key.as_bytes());
            // importing our certificate for CA
            let s=include_str!("../my_ca.pem");
            // converting it into a certificate
            let ca=tonic::transport::Certificate::from_pem(s.as_bytes());
            // telling our client what is the identity of our server
            let tls=tonic::transport::ClientTlsConfig::new().domain_name("localhost").identity(id).ca_certificate(ca);
            // connecting with tls
            let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
                .tls_config(tls)
                .connect()
                .await?;
    

Enter fullscreen mode Exit fullscreen mode

This shows how to configure for TLS.  

            let say = MySay::default();
        // reading cert and key disk
            let cert = include_str!("../server.pem");
            let key = include_str!("../server.key");
        // creating identity from cert and key
            let id = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());
        // reading ca root from disk
            let s = include_str!("../my_ca.pem");
        // creating certificate
            let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
        // creating tls config
            let tls = tonic::transport::ServerTlsConfig::new()
                .identity(id)
                .client_ca_root(ca);
        // creating server with tls
            Server::builder()
                .tls_config(tls)
                .add_service(ser)
                .serve(addr)
                .await?;
            Ok(())
    
    

Enter fullscreen mode Exit fullscreen mode

[](#conclusion)Conclusion
=========================

We have gone through basic protocol buffer and gRPC. We have created our server. We also created a client for interacting with the server. We learned how to compile our `.proto` file in rust client. We also learned how to stream responses and requests. We also created a bidirectional stream. We learned two different authentication strategy. We implemented JWT and Mutual TLS based authentication. Now you have a basic understanding of gRPC, you can create your own micro-service based app. gRPC comes with support for load-balancing,tracing and health tracking. Now you can explore further functionality of gRPC.

[](#code-sample)Code Sample
===========================

[https://github.com/anshulrgoyal/rust-grpc-demo](https://github.com/anshulrgoyal/rust-grpc-demo)

Discussion (18)
---------------

Subscribe

    ![pic](https://res.cloudinary.com/practicaldev/image/fetch/s--RmY55OKL--/c_limit,f_auto,fl_progressive,q_auto,w_256/https://practicaldev-herokuapp-com.freetls.fastly.net/assets/devlogo-pwa-512.png) 

   Upload image    

Templates [Editor guide](/p/editor_guide "Markdown Guide")

Personal Moderator

![loading](https://dev.to/assets/loading-ellipsis-b714cf681fd66c853ff6f03dd161b77aa3c80e03cdc06f478b695f42770421e9.svg)

[Create template](/settings/response-templates)

Templates let you quickly answer FAQs or store snippets for re-use.

Submit Preview [Dismiss](/404.html)

Collapse Expand

 [![wotzhs profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--nzdhW84x--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/262303/023fe862-c983-43b8-a4a7-11fe31279c2e.png)](https://dev.to/wotzhs) 

[Sean Wong](https://dev.to/wotzhs)

Sean Wong

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--ipXV5ZHY--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/262303/023fe862-c983-43b8-a4a7-11fe31279c2e.png) Sean Wong](/wotzhs) 

Follow

*   Joined
    
    31 Oct 2019
    

• [Nov 11 '20](https://dev.to/wotzhs/comment/17oj2)

Dropdown menu

*   [Copy link](https://dev.to/wotzhs/comment/17oj2)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/wotzhs/comment/17oj2)

Hi Anshul, I am very new to Rust development, i find this article extremely helpful, however I am not quite clear with the statement below:

> This will help us testing and maintaining code in save repo but it is not suggested for a large project

is this referring to the bin block in the `cargo.toml`? is so, may I know what would be the proper way to run the rust grpc server?

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/17oj2)

Collapse Expand

 [![anshulgoyal15 profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--bdsCQQ4i--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/192056/643512e0-f55c-4465-a7d0-7762199281b2.png)](https://dev.to/anshulgoyal15) 

[Anshul Goyal Author](https://dev.to/anshulgoyal15)

Anshul Goyal

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--n4m_l7P6--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/192056/643512e0-f55c-4465-a7d0-7762199281b2.png) Anshul Goyal](/anshulgoyal15) 

Follow

*   Joined
    
    8 Jul 2019
    

Author

• [Nov 14 '20](https://dev.to/anshulgoyal15/comment/18176)

Dropdown menu

*   [Copy link](https://dev.to/anshulgoyal15/comment/18176)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/anshulgoyal15/comment/18176)

Yes, the better way is to use cargo wrokspaces.

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/18176)

Collapse Expand

 [![wotzhs profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--nzdhW84x--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/262303/023fe862-c983-43b8-a4a7-11fe31279c2e.png)](https://dev.to/wotzhs) 

[Sean Wong](https://dev.to/wotzhs)

Sean Wong

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--ipXV5ZHY--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/262303/023fe862-c983-43b8-a4a7-11fe31279c2e.png) Sean Wong](/wotzhs) 

Follow

*   Joined
    
    31 Oct 2019
    

• [Nov 16 '20](https://dev.to/wotzhs/comment/183b3)

Dropdown menu

*   [Copy link](https://dev.to/wotzhs/comment/183b3)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/wotzhs/comment/183b3)

Thanks Anshul, I have been looking at different gRPC crates and I think another way that to not use the `cargo run --bin server` is to use the statically generated code from `protoc --rust_out` and use them to build the grpc server manually.

Particularly I found [github.com/tikv/grpc-rs](https://github.com/tikv/grpc-rs).

I understood completely that this article was meant to demonstrate the dynamically genarated grpc server & client, so I hope this comment is not to be taken in the wrong light.

Like comment: Like comment: 2 likes Thread Thread

 [![anshulgoyal15 profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--bdsCQQ4i--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/192056/643512e0-f55c-4465-a7d0-7762199281b2.png)](https://dev.to/anshulgoyal15) 

[Anshul Goyal Author](https://dev.to/anshulgoyal15)

Anshul Goyal

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--n4m_l7P6--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/192056/643512e0-f55c-4465-a7d0-7762199281b2.png) Anshul Goyal](/anshulgoyal15) 

Follow

*   Joined
    
    8 Jul 2019
    

Author

• [Nov 16 '20](https://dev.to/anshulgoyal15/comment/183b9)

Dropdown menu

*   [Copy link](https://dev.to/anshulgoyal15/comment/183b9)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/anshulgoyal15/comment/183b9)

Hi, Sean I always like to generate stubs at build time. It allows us to maintain a sync between protocol buffer definations and stubs. I think it is best practice to not to generate stubs before hand.

Like comment: Like comment: 2 likes Thread Thread

 [![wotzhs profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--nzdhW84x--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/262303/023fe862-c983-43b8-a4a7-11fe31279c2e.png)](https://dev.to/wotzhs) 

[Sean Wong](https://dev.to/wotzhs)

Sean Wong

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--ipXV5ZHY--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/262303/023fe862-c983-43b8-a4a7-11fe31279c2e.png) Sean Wong](/wotzhs) 

Follow

*   Joined
    
    31 Oct 2019
    

• [Nov 16 '20](https://dev.to/wotzhs/comment/183bf)

Dropdown menu

*   [Copy link](https://dev.to/wotzhs/comment/183bf)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/wotzhs/comment/183bf)

yup, agreed, generating stub at build time guarantees the latest protobuf code, that's my preference too.

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/183bf)

Collapse Expand

 [![marshalshi profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--uUKB2jnl--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/539852/8eb2e263-7338-43c3-aa96-741470f4360e.png)](https://dev.to/marshalshi) 

[MarshalSHI](https://dev.to/marshalshi)

MarshalSHI

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--Xea14kDN--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/539852/8eb2e263-7338-43c3-aa96-741470f4360e.png) MarshalSHI](/marshalshi) 

Follow

*   Joined
    
    14 Dec 2020
    

• [Mar 5](https://dev.to/marshalshi/comment/1c604)

Dropdown menu

*   [Copy link](https://dev.to/marshalshi/comment/1c604)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/marshalshi/comment/1c604)

Hi Anshul,

Nice post. Thanks.

A question about CA. You used self-signed CA. and I did same like your posted. But my client cannot talk to server. It tells me `transport error`.

After checking online, I found that `libwebpki` said they are not supporting self-signed CA currently. (Link is here: [github.com/briansmith/webpki/issue...](https://github.com/briansmith/webpki/issues/114#issuecomment-759842242))

Could you describe more for CA part? Thanks

Best,  
Marshal

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/1c604)

Collapse Expand

 [![vixorem profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--BMsJlF7L--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/622142/778c4f0e-014f-43c8-b495-a28983880c54.jpeg)](https://dev.to/vixorem) 

[Victor](https://dev.to/vixorem)

Victor

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--q4772aqL--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/622142/778c4f0e-014f-43c8-b495-a28983880c54.jpeg) Victor](/vixorem) 

Follow

*   Joined
    
    28 Apr 2021
    

• [May 30 • Edited on May 30](https://dev.to/vixorem/comment/1f0pc)

Dropdown menu

*   [Copy link](https://dev.to/vixorem/comment/1f0pc)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/vixorem/comment/1f0pc)

HI! I'm having a trouble with streaming. I cloned your repo and ran server and cient. I tried function send\_stream but modified it putting sleep(4 second) for each iteration in the spawned task. When I ran client I was expecting a short delay between messages I receive from the server but it worked differently. The delay took about 20 seconds without any incoming messages (4 iterations with 4 sleeps per eacn) and then I got all the messages at the moment. Why does it happen and how can I make streaming send and receive in time?

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/1f0pc)

Collapse Expand

 [![vixorem profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--BMsJlF7L--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/622142/778c4f0e-014f-43c8-b495-a28983880c54.jpeg)](https://dev.to/vixorem) 

[Victor](https://dev.to/vixorem)

Victor

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--q4772aqL--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/622142/778c4f0e-014f-43c8-b495-a28983880c54.jpeg) Victor](/vixorem) 

Follow

*   Joined
    
    28 Apr 2021
    

• [May 30](https://dev.to/vixorem/comment/1f0pn)

Dropdown menu

*   [Copy link](https://dev.to/vixorem/comment/1f0pn)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/vixorem/comment/1f0pn)

I've set channel buffer size to 1 and it seems to be working but I don't undestand why

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/1f0pn)

Collapse Expand

 [![dpineiden profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--EGez_pGg--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/52225/99de66d6-1ce7-43ff-ba50-edc94c7f71b2.jpeg)](https://dev.to/dpineiden) 

[David Pineda](https://dev.to/dpineiden)

David Pineda

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--o2mySBXq--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/52225/99de66d6-1ce7-43ff-ba50-edc94c7f71b2.jpeg) David Pineda](/dpineiden) 

Follow

*   Joined
    
    2 Jan 2018
    

• [Nov 15](https://dev.to/dpineiden/comment/1jljl)

Dropdown menu

*   [Copy link](https://dev.to/dpineiden/comment/1jljl)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/dpineiden/comment/1jljl)

Hi!  
Very thanks for your tutorial. Not so simple, not so complex. The just point.  
Now I recommed to update the code for this days, changes a bit. I've shared my repo  
[gitlab.com/pineiden/gprc-tonic-rust](https://gitlab.com/pineiden/gprc-tonic-rust)  
BR

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/1jljl)

Collapse Expand

 [![myleftfoot profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--1XRenIEq--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/455569/0c078022-8652-4d74-9c14-1e7ba780c947.jpeg)](https://dev.to/myleftfoot) 

[Stéphane Trottier](https://dev.to/myleftfoot)

Stéphane Trottier

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--wggny1Dy--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/455569/0c078022-8652-4d74-9c14-1e7ba780c947.jpeg) Stéphane Trottier](/myleftfoot) 

Follow

*   Joined
    
    20 Aug 2020
    

• [Aug 20 '20](https://dev.to/myleftfoot/comment/13o47)

Dropdown menu

*   [Copy link](https://dev.to/myleftfoot/comment/13o47)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/myleftfoot/comment/13o47)

good sample, had to tweak the JWT sample code.

had to change  

`mut req: Request`  

to  

`mut req: tonic::Request`  

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/13o47)

Collapse Expand

 [![akhilerm profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--tHpCJ_84--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/406864/9649ee25-22ae-4c80-a82a-e9ffb6b23de8.jpeg)](https://dev.to/akhilerm) 

[Akhil Mohan](https://dev.to/akhilerm)

Akhil Mohan

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--5z3W5MXB--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/406864/9649ee25-22ae-4c80-a82a-e9ffb6b23de8.jpeg) Akhil Mohan](/akhilerm) 

Follow

*   Work
    
    Software Engineer
    
*   Joined
    
    11 Jun 2020
    

• [Jul 3 '20](https://dev.to/akhilerm/comment/11bll)

Dropdown menu

*   [Copy link](https://dev.to/akhilerm/comment/11bll)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/akhilerm/comment/11bll)

Can you point to the complete code repository ?

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/11bll)

Collapse Expand

 [![mikkelhjuul profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--K7fNPoXB--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/483052/c6178043-7656-41ec-bcf1-66164b9454a3.png)](https://dev.to/mikkelhjuul) 

[MikkelHJuul](https://dev.to/mikkelhjuul)

MikkelHJuul

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--s-teKSyh--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/483052/c6178043-7656-41ec-bcf1-66164b9454a3.png) MikkelHJuul](/mikkelhjuul) 

Follow

*   Joined
    
    5 Oct 2020
    

• [Oct 5 '20](https://dev.to/mikkelhjuul/comment/1671n)

Dropdown menu

*   [Copy link](https://dev.to/mikkelhjuul/comment/1671n)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/mikkelhjuul/comment/1671n)

He would have to add it, because it doesn't seem to be on his github.  
for some small points, the name of the `impl` has to be `Say` (given by the `.proto`), the file `hell.rs` should probably have been `hello.rs` and is placed in folder `src`. This is my findings so far.  
This should bring you to a state where if you have not added anything to your implementation the compiler will spit at you: `not all trait items implemented, missing: ...` which makes perfect sense

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/1671n)

Collapse Expand

 [![anshulgoyal15 profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--bdsCQQ4i--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/192056/643512e0-f55c-4465-a7d0-7762199281b2.png)](https://dev.to/anshulgoyal15) 

[Anshul Goyal Author](https://dev.to/anshulgoyal15)

Anshul Goyal

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--n4m_l7P6--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/192056/643512e0-f55c-4465-a7d0-7762199281b2.png) Anshul Goyal](/anshulgoyal15) 

Follow

*   Joined
    
    8 Jul 2019
    

Author

• [Oct 25 '20](https://dev.to/anshulgoyal15/comment/17846)

Dropdown menu

*   [Copy link](https://dev.to/anshulgoyal15/comment/17846)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/anshulgoyal15/comment/17846)

Here is the repo [github.com/anshulrgoyal/rust-grpc-...](https://github.com/anshulrgoyal/rust-grpc-demo)

Like comment: Like comment: 1 like Thread Thread

 [![naveendavis11 profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--Ty8SaK2c--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/362261/cf48cb82-0d13-4228-bd79-da9582a4b7b8.jpg)](https://dev.to/naveendavis11) 

[Naveen Davis](https://dev.to/naveendavis11)

Naveen Davis

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--EjGRAjeE--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/362261/cf48cb82-0d13-4228-bd79-da9582a4b7b8.jpg) Naveen Davis](/naveendavis11) 

Follow

Programmer

*   Location
    
    Vancouver
    
*   Work
    
    Programmer
    
*   Joined
    
    7 Apr 2020
    

• [Feb 22](https://dev.to/naveendavis11/comment/1bl67)

Dropdown menu

*   [Copy link](https://dev.to/naveendavis11/comment/1bl67)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/naveendavis11/comment/1bl67)

I was using 2 bin. But  
hello module is not getting imported to client.rs.  
  
My src folder structure  
client.rs  
server.rs  
hello.rs

Not sure whether I need to make lib.rs

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/1bl67)

Collapse Expand

 [![anshulgoyal15 profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--bdsCQQ4i--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/192056/643512e0-f55c-4465-a7d0-7762199281b2.png)](https://dev.to/anshulgoyal15) 

[Anshul Goyal Author](https://dev.to/anshulgoyal15)

Anshul Goyal

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--n4m_l7P6--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/192056/643512e0-f55c-4465-a7d0-7762199281b2.png) Anshul Goyal](/anshulgoyal15) 

Follow

*   Joined
    
    8 Jul 2019
    

Author

• [Oct 25 '20](https://dev.to/anshulgoyal15/comment/17845)

Dropdown menu

*   [Copy link](https://dev.to/anshulgoyal15/comment/17845)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/anshulgoyal15/comment/17845)

Hi  
Sorry for the late reply.  
Here is the code repo [github.com/anshulrgoyal/rust-grpc-...](https://github.com/anshulrgoyal/rust-grpc-demo)

Like comment: Like comment: 2 likes [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/17845)

Collapse Expand

 [![akhilerm profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--tHpCJ_84--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/406864/9649ee25-22ae-4c80-a82a-e9ffb6b23de8.jpeg)](https://dev.to/akhilerm) 

[Akhil Mohan](https://dev.to/akhilerm)

Akhil Mohan

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--5z3W5MXB--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/406864/9649ee25-22ae-4c80-a82a-e9ffb6b23de8.jpeg) Akhil Mohan](/akhilerm) 

Follow

*   Work
    
    Software Engineer
    
*   Joined
    
    11 Jun 2020
    

• [Nov 6 '20](https://dev.to/akhilerm/comment/17jpb)

Dropdown menu

*   [Copy link](https://dev.to/akhilerm/comment/17jpb)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/akhilerm/comment/17jpb)

Thank you.

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/17jpb)

Collapse Expand

 [![geoxion profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--ZK-uHJYc--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/296009/7deb2e84-c1af-4222-b182-86750b3f6140.jpg)](https://dev.to/geoxion) 

[Dion Dokter](https://dev.to/geoxion)

Dion Dokter

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--yG1ex7C4--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/296009/7deb2e84-c1af-4222-b182-86750b3f6140.jpg) Dion Dokter](/geoxion) 

Follow

Working on innovation in the steel industry. Lover of the Rust language.

*   Location
    
    Netherlands
    
*   Work
    
    Jr. Software Engineer at Netherlands
    
*   Joined
    
    18 Dec 2019
    

• [May 6 '20](https://dev.to/geoxion/comment/ohe2)

Dropdown menu

*   [Copy link](https://dev.to/geoxion/comment/ohe2)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/geoxion/comment/ohe2)

Very good post! This is exactly what I needed 😁

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/ohe2)

Collapse Expand

 [![shionryuu profile image](https://res.cloudinary.com/practicaldev/image/fetch/s--VHWqnFfj--/c_fill,f_auto,fl_progressive,h_50,q_auto,w_50/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/480917/9ccb0329-e518-4d42-9341-00e522d2608f.jpeg)](https://dev.to/shionryuu) 

[Shion Ryuu](https://dev.to/shionryuu)

Shion Ryuu

 [![](https://res.cloudinary.com/practicaldev/image/fetch/s--ksBSFMwQ--/c_fill,f_auto,fl_progressive,h_90,q_auto,w_90/https://dev-to-uploads.s3.amazonaws.com/uploads/user/profile_image/480917/9ccb0329-e518-4d42-9341-00e522d2608f.jpeg) Shion Ryuu](/shionryuu) 

Follow

*   Joined
    
    3 Oct 2020
    

• [Oct 22 '20](https://dev.to/shionryuu/comment/17506)

Dropdown menu

*   [Copy link](https://dev.to/shionryuu/comment/17506)

*   Hide

*   [Report abuse](/report-abuse?url=https://dev.to/shionryuu/comment/17506)

title "Token-Based Authentication" is not correct show.

"use" is missing before "jsonwebtoken crate".

Like comment: Like comment: 1 like [Comment button Reply](#/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o/comments/new/17506)

[Code of Conduct](/code-of-conduct) • [Report abuse](/report-abuse)

Are you sure you want to hide this comment? It will become hidden in your post, but will still be visible via the comment's [permalink](#).

Hide child comments as well

Confirm

For further actions, you may consider blocking this person and/or [reporting abuse](/report-abuse)

var waitingOnPodcast = setInterval(function() { if (typeof initializePodcastPlayback !== 'undefined') { initializePodcastPlayback(); clearInterval(waitingOnPodcast); } }, 1); if (document.head.querySelector('meta\[name="user-signed-in"\]\[content="true"\]')) { function displayPollResults(json) { var totalVotes = json.voting\_data.votes\_count; json.voting\_data.votes\_distribution.forEach(function(point) { var pollOptionItem = document.getElementById('poll\_option\_list\_item\_'+point\[0\]); var optionText = document.getElementById('poll\_option\_label\_'+point\[0\]).textContent; if (json.user\_vote\_poll\_option\_id === point\[0\]) { var votedClass = 'optionvotedfor' } else { var votedClass = 'optionnotvotedfor' } if (totalVotes === 0) { var percent = 0; } else { var percent = (point\[1\]/totalVotes)\*100; } var roundedPercent = Math.round( percent \* 10 ) / 10 var percentFromRight = (100-roundedPercent) var html = '<span><span class="ltag-votepercent ltag-'+votedClass+'" style="right:'+percentFromRight+'%"></span> <span class="ltag-votepercenttext">'+optionText+' — '+roundedPercent+'%</span></span>'; pollOptionItem.innerHTML = html; pollOptionItem.classList.add('already-voted') document.getElementById('showmethemoney-'+json.poll\_id).innerHTML = '<span class="ltag-voting-results-count">'+totalVotes+' total votes</span>'; }) } var polls = document.getElementsByClassName('ltag-poll'); for (var i = 0; i < polls.length; i += 1) { var poll = polls\[i\] var pollId = poll.dataset.pollId window.fetch('/poll\_votes/'+pollId) .then(function(response){ response.json().then( function(json) { if (json.voted) { displayPollResults(json) } else { var els = document.getElementById('poll\_'+json.poll\_id).getElementsByClassName('ltag-polloption'); for (i = 0; i < els.length; i += 1) { els\[i\].addEventListener('click', function(e) { var tokenMeta = document.querySelector("meta\[name='csrf-token'\]") if (!tokenMeta) { alert('Whoops. There was an error. Your vote was not counted. Try refreshing the page.') return } var csrfToken = tokenMeta.getAttribute('content') var optionId = e.target.dataset.optionId window.fetch('/poll\_votes', { method: 'POST', headers: { 'X-CSRF-Token': csrfToken, 'Content-Type': 'application/json', }, body: JSON.stringify({poll\_vote: { poll\_option\_id: optionId } }), credentials: 'same-origin', }).then(function(response){ response.json().then(function(j){displayPollResults(j)}) }) }); } document.getElementById('showmethemoney-'+json.poll\_id).addEventListener('click', function() { pollId = this.dataset.pollId window.fetch('/poll\_skips', { method: 'POST', headers: { 'X-CSRF-Token': csrfToken, 'Content-Type': 'application/json', }, body: JSON.stringify({poll\_skip: {poll\_id: pollId }}), credentials: 'same-origin', }).then(function(response){ response.json().then(function(j){displayPollResults(j)}) }) }); } } ) }) } } else { var els = document.getElementsByClassName('ltag-poll') for (i = 0; i < els.length; i += 1) { els\[i\].onclick = function(e) { if (typeof showLoginModal !== "undefined") { showLoginModal(); } } } } function activateRunkitTags() { if (!areAnyRunkitTagsPresent()) return var checkRunkit = setInterval(function() { try { dynamicallyLoadRunkitLibrary() if (typeof(RunKit) === 'undefined') { return } replaceTagContentsWithRunkitWidget() clearInterval(checkRunkit); } catch(e) { console.error(e); clearInterval(checkRunkit); } }, 200); } function isRunkitTagAlreadyActive(runkitTag) { return runkitTag.querySelector("iframe") !== null; }; function areAnyRunkitTagsPresent() { var presentRunkitTags = document.getElementsByClassName("runkit-element"); return presentRunkitTags.length > 0 } function replaceTagContentsWithRunkitWidget() { var targets = document.getElementsByClassName("runkit-element"); for (var i = 0; i < targets.length; i++) { if (isRunkitTagAlreadyActive(targets\[i\])) { continue; } var wrapperContent = targets\[i\].textContent; if (/^(<iframe src)/.test(wrapperContent) === false) { if (targets\[i\].children.length > 0) { var preamble = targets\[i\].children\[0\].textContent; var content = targets\[i\].children\[1\].textContent; targets\[i\].innerHTML = ""; var notebook = RunKit.createNotebook({ element: targets\[i\], source: content, preamble: preamble }); } } } }; function dynamicallyLoadRunkitLibrary() { if (typeof(dynamicallyLoadScript) === "undefined") return dynamicallyLoadScript("//embed.runkit.com") } activateRunkitTags(); var videoPreviews = document.getElementsByClassName("ltag\_\_twitter-tweet\_\_media\_\_video-wrapper"); \[\].forEach.call(videoPreviews, function(el) { el.onclick = function(e) { var divHeight = el.offsetHeight; el.style.maxHeight = divHeight + "px"; el.getElementsByClassName("ltag\_\_twitter-tweet\_\_media--video-preview")\[0\].style.display = "none"; el.getElementsByClassName("ltag\_\_twitter-tweet\_\_video")\[0\].style.display = "block"; el.getElementsByTagName("video")\[0\].play(); } }); var tweets = document.getElementsByClassName("ltag\_\_twitter-tweet\_\_main"); \[\].forEach.call(tweets, function(tweet){ tweet.onclick = function(e) { if (e.target.nodeName == "A" || e.target.parentElement.nodeName == "A") { return; } window.open(tweet.dataset.url,"\_blank"); } }); var subscribeBtn = document.getElementById('subscribe-btn'); function isUserSignedIn() { return document.head.querySelector('meta\[name="user-signed-in"\]\[content="true"\]') !== null; } // Hiding/showing elements // If clearSubscribeButton is false, we will not clear out the subscription-signed-in area, // which will allow users to re-submit their subscription if they see any error messages. // \*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\* function clearSubscriptionArea({ clearSubscribeButton = true } = {}) { if (!clearSubscribeButton) { // Allow users to try submitting again if they see an error. hideSubscriptionSignedIn(); } const subscriptionSignedOut = document.getElementById('subscription-signed-out'); if (subscriptionSignedOut) { subscriptionSignedOut.classList.add("hidden"); } hideResponseMessage(); const subscriberAppleAuth = document.getElementById('subscriber-apple-auth'); if (subscriberAppleAuth) { subscriberAppleAuth.classList.add("hidden"); } hideConfirmationModal(); } // Hides the response message (which displays success/error messages) if it exists. // \*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\* function hideResponseMessage() { const responseMessage = document.getElementById('response-message'); if (responseMessage) { responseMessage.classList.add("hidden"); } } function showSignedIn() { clearSubscriptionArea(); const subscriptionSignedIn = document.getElementById('subscription-signed-in'); if (subscriptionSignedIn) { subscriptionSignedIn.classList.remove("hidden"); } const profileImages = document.getElementById('profile-images'); if (profileImages) { profileImages.classList.remove("signed-out"); profileImages.classList.add("signed-in"); } } function showSignedOut() { clearSubscriptionArea(); const subscriptionSignedOut = document.getElementById('subscription-signed-out'); if (subscriptionSignedOut) { subscriptionSignedOut.classList.remove("hidden"); } const profileImages = document.getElementById('profile-images'); if (profileImages) { profileImages.classList.remove("signed-in"); profileImages.classList.add("signed-out"); } const subscriberProfileImage = document.getElementsByClassName('ltag\_\_user-subscription-tag\_\_subscriber-profile-image')\[0\]; if (subscriberProfileImage) { subscriberProfileImage.classList.add("hidden"); } } function showResponseMessage(noticeType, msg) { clearSubscriptionArea(clearSubscribeButton = false); const responseMessage = document.getElementById('response-message'); if (responseMessage) { responseMessage.classList.remove("hidden"); responseMessage.classList.add(\`crayons-notice--${noticeType}\`); responseMessage.textContent = msg; if (noticeType === 'danger') { // Allow users to try resubscribing if they see an error message. subscribeBtn.textContent = "Submit"; subscribeBtn.disabled = false; } } } function showAppleAuthMessage() { clearSubscriptionArea(); const subscriber = userData(); if (subscriber) { updateProfileImages('.ltag\_\_user-subscription-tag\_\_subscriber-profile-image', subscriber); } const subscriberAppleAuth = document.getElementById('subscriber-apple-auth'); if (subscriberAppleAuth) { subscriberAppleAuth.classList.remove("hidden"); } } function showSubscribed() { hideSubscriptionSignedIn(); updateSubscriberData(); const authorUsername = document.getElementById('user-subscription-tag')?.dataset.authorUsername; const alreadySubscribedMsg = \`You are already subscribed.\`; showResponseMessage('success', alreadySubscribedMsg); } function showConfirmationModal() { const confirmationModal = document.getElementById('user-subscription-confirmation-modal'); if (confirmationModal) { confirmationModal.classList.remove("hidden"); } } function hideConfirmationModal() { const confirmationModal = document.getElementById('user-subscription-confirmation-modal'); if (confirmationModal) { confirmationModal.classList.add("hidden"); } } // Updating DOM elements // \*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\* function updateSubscriberData() { const subscriber = userData(); if (subscriber.email) { updateElementsTextContent('.ltag\_\_user-subscription-tag\_\_subscriber-email', subscriber.email); } updateProfileImages('.ltag\_\_user-subscription-tag\_\_subscriber-profile-image', subscriber); } function updateElementsTextContent(identifier, value) { const elements = document.querySelectorAll(identifier); elements.forEach(function(element) { element.textContent = value; }); } function updateProfileImages(identifier, subscriber) { const profileImages = document.querySelectorAll(\`img${identifier}\`); profileImages.forEach(function(profileImage) { profileImage.src = subscriber.profile\_image\_90; profileImage.alt = \`${subscriber.username} profile image\`; }); } function hideSubscriptionSignedIn() { const subscriptionSignedIn = document.getElementById('subscription-signed-in'); if (subscriptionSignedIn) { subscriptionSignedIn.classList.add("hidden"); } } // Adding event listeners for 'click' // \*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\* function addSignInClickHandler() { const signInBtn = document.getElementById('sign-in-btn'); if (signInBtn) { signInBtn.addEventListener('click', function(e) { if (typeof showLoginModal !== 'undefined') { showLoginModal(); } }); } } function addConfirmationModalClickHandlers() { if (subscribeBtn) { subscribeBtn.addEventListener('click', function(e) { showConfirmationModal(); }); } const cancelBtn = document.getElementById('cancel-btn'); if (cancelBtn) { cancelBtn.addEventListener('click', function(e) { hideConfirmationModal(); }); } const closeConfirmationModal = document.getElementById('close-confirmation-modal'); if (closeConfirmationModal) { closeConfirmationModal.addEventListener('click', function(e) { hideConfirmationModal(); }); } const confirmationModal = document.getElementById('confirmation-btn') if (confirmationModal) { confirmationModal.addEventListener('click', function(e) { handleSubscription(); }); } } // API calls // \*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\* function submitSubscription() { // Hide any error messages previously rendered. hideResponseMessage(); subscribeBtn.textContent = "Submitting..."; subscribeBtn.disabled = true; const headers = { Accept: 'application/json', 'X-CSRF-Token': window.csrfToken, 'Content-Type': 'application/json', } const articleBody = document.getElementById('article-body'); const articleId = (articleBody ? articleBody.dataset.articleId : null); const subscriber = userData(); const body = JSON.stringify( { user\_subscription: { source\_type: 'Article', source\_id: articleId, subscriber\_email: subscriber.email } } ) return fetch('/user\_subscriptions', { method: 'POST', headers: headers, credentials: 'same-origin', body: body, }).then(function(response) { return response.json(); }); } function fetchIsSubscribed() { const articleBody = document.getElementById('article-body'); const articleId = (articleBody ? articleBody.dataset.articleId : null); const params = new URLSearchParams({ source\_type: 'Article', source\_id: articleId }).toString(); const headers = { Accept: 'application/json', 'X-CSRF-Token': window.csrfToken, 'Content-Type': 'application/json', } return fetch(\`/user\_subscriptions/subscribed?${params}\`, { method: 'GET', headers: headers, credentials: 'same-origin', }).then(function(response) { if (response.ok) { return response.json(); } else { console.error(\`Base data error: ${response.status} - ${response.statusText}\`); } }); } // Handle API responses // \*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\* function handleSubscription() { hideConfirmationModal(); // Close the modal once the user has confirmed. submitSubscription().then(function(response) { if (response.success) { const userSubscriptionTag = document.getElementById('user-subscription-tag'); const authorUsername = (userSubscriptionTag ? userSubscriptionTag.dataset.authorUsername : null); const successMsg = \`You are now subscribed and may receive emails from ${authorUsername}\`; showResponseMessage('success', successMsg); hideSubscriptionSignedIn(); } else { showResponseMessage('danger', response.error); } }); } function checkIfSubscribed() { fetchIsSubscribed().then(function(response) { const subscriber = userData(); const isSubscriberAuthedWithApple = (subscriber.email ? subscriber.email.endsWith('@privaterelay.appleid.com') : false); if (response.is\_subscribed) { showSubscribed(); } else if (isSubscriberAuthedWithApple) { showAppleAuthMessage(); } else { updateSubscriberData(); } }); } // We load this JS on every Article. This is to only run it on Articles // where the UserSubscription liquid tag is present if (document.getElementById('user-subscription-tag')) { // The markup defaults to signed out UX if (isUserSignedIn()) { showSignedIn(); addConfirmationModalClickHandlers(); // We need access to some DOM elements (i.e. csrf token, article id, userData, etc.) document.addEventListener('DOMContentLoaded', function() { checkIfSubscribed(); }); } else { showSignedOut(); addSignInClickHandler(); } }

[DEV Community – A constructive and inclusive social network for software developers. With you every step of your journey.](/)

[](/)

[](/)

[Built on](/) [Forem](https://www.forem.com) — the [open source](https://dev.to/t/opensource) software that powers [DEV](https://dev.to) and other inclusive communities.

Made with love and [Ruby on Rails](https://dev.to/t/rails). DEV Community © 2016 - 2021.

[Forem logo](https://www.forem.com)

![DEV Community](https://res.cloudinary.com/practicaldev/image/fetch/s--pcSkTMZL--/c_limit,f_auto,fl_progressive,q_80,w_190/https://practicaldev-herokuapp-com.freetls.fastly.net/assets/devlogo-pwa-512.png)

We're a place where coders share, stay up-to-date and grow their careers.

[Log in](/enter) [Create account](/enter?state=new-user)