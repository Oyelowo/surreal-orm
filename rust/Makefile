#############
## STANDARD
#############
install:
	cargo install cargo-edit  

upgrade:
	cargo upgrade

sync:
# Update graphql schemas which is then consumed by the frontend graphql build script
	cargo run -p graphql-surrealdb --bin graphql_generator_graphql_surrealdb

dev:
	cargo run -p graphql-surrealdb --bin graphql-surrealdb

format:
	cargo fmt --all -- --check
	cargo clippy --fix
	
check:
	cargo clippy -- -D  warnings

test:
	cargo test

test-watch:
	cargo watch -x 'test --offline -- --color=always'

build:
	cargo build

run:
	cargo run -p graphql-surrealdb --bin graphql-surrealdb

run-surrealdb:
	# docker run --rm -p 8000:8000 surrealdb/surrealdb:latest start
	docker run --rm --name surrealdb -p 127.0.0.1:8000:8000 surrealdb/surrealdb:latest start --log trace --user root --pass root memory
