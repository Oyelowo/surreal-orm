# Build stage
FROM rust:1.57 as builder
WORKDIR /app
#ADD . /app

# COPY --from=shared /app .
COPY ./Cargo.lock .
COPY ./Cargo.toml .
ADD common .
ADD backend-main .

RUN cargo build -p backend-main --bin backend-main


FROM gcr.io/distroless/cc as runtime
WORKDIR /app
# COPY --from=builder /app/target/release/backend-main /usr/local/bin
COPY --from=builder /app/target/release/backend-main ./
ENV RUST_ENV=local
EXPOSE 8000
# ENTRYPOINT ["/usr/local/bin/backend-main"]
CMD ["./backend-main"]
