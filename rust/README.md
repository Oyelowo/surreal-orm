# Build graphql-surrealdb
```bash
docker build . --target graphql-surrealdb  -t oyelowo/graphql-surrealdb

## Run
docker run -p 8000:8000 -e RUST_ENV=local oyelowo/graphql-surrealdb
```

# Build grpc-surrealdb
```bash
docker build . --target grpc-surrealdb  -t oyelowo/grpc-surrealdb

## Run
docker run -p 8000:8000 oyelowo/grpc-surrealdb
```bash
