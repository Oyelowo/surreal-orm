ARG BASE_IMAGE=rust:1.57

FROM $BASE_IMAGE as rust-with-chef
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.33

FROM rust-with-chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust-with-chef as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


FROM $BASE_IMAGE AS shared
WORKDIR /app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

## FIRST BACKEND SERVICE
FROM shared AS builder-backend-main 
RUN cargo build --release -p backend-main

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/cc AS backend-main
WORKDIR /app
COPY --from=builder-backend-main /app/target/release/backend-main ./
ENV RUST_ENV=local
ENTRYPOINT ["./backend-main"]

## SECOND BACKEND SERVICE
FROM shared AS builder-second
RUN rustup component add rustfmt
RUN cargo build --release -p backend-second

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/cc AS backend-second
WORKDIR /app
COPY --from=builder-second app/target/release/backend-second ./
ENTRYPOINT ["./backend-second"]