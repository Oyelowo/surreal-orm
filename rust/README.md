# Build graphql-mongo
```bash
docker build . --target graphql-mongo  -t oyelowo/graphql-mongo

## Run
docker run -p 8000:8000 -e RUST_ENV=local oyelowo/graphql-mongo
```


# Build grpc-mongo
```bash
docker build . --target grpc-mongo  -t oyelowo/grpc-mongo

## Run
docker run -p 50051:50051 oyelowo/grpc-mongo
```bash
