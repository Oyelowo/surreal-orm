# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.57.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


## DEPENDENNCIES
FROM chef AS builder-dependencies-only
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .


## FIRST BACKEND SERVICE
FROM builder-dependencies-only AS builder-backend-main 
RUN cargo build --release -p backend-main

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/cc AS backend-main
WORKDIR /app
COPY --from=builder-backend-main /app/target/release/backend-main /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]

## SECOND BACKEND SERVICE
FROM builder-dependencies-only AS builder-second
RUN rustup component add rustfmt
RUN cargo build --release -p backend-second

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/cc AS backend-second
WORKDIR /app
COPY --from=builder-second app/target/release/backend-main /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]