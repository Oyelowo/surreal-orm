# Build backend-main
```bash
docker build . --target backend-main  -t oyelowo/backend-main

## Run
docker run -p 8000:8000 -e RUST_ENV=local oyelowo/backend-main
```


# Build backend-second
```bash
docker build . --target backend-second  -t oyelowo/backend-second

## Run
docker run -p 50051:50051 oyelowo/backend-second
```bash
