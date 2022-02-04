grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' localhost:50051 helloworld.Greeter/SayHello


BloomRPC