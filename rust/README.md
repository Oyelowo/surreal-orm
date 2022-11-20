# Build app-graphql-surrealdb
```bash
docker build . --target app-graphql-surrealdb  -t oyelowo/app-graphql-surrealdb

## Run
docker run -p 8000:8000 -e RUST_ENV=local oyelowo/app-graphql-surrealdb
```

# Build app-grpc-surrealdb
```bash
docker build . --target app-grpc-surrealdb  -t oyelowo/app-grpc-surrealdb

## Run
docker run -p 8000:8000 oyelowo/app-grpc-surrealdb
```bash

