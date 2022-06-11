gen:
	make generate-graphql-mongo-graphql-schema
	# cargo run -p graphql-mongo --bin graphql_generator

fmt:
	SQLX_OFFLINE=true cargo fmt --all -- --check
	SQLX_OFFLINE=true cargo clippy --fix

clippy:
	SQLX_OFFLINE=true cargo clippy -- -D  warnings

clippy-fix:
	SQLX_OFFLINE=true cargo clippy --fix

test:
	SQLX_OFFLINE=true cargo test

# 1. graphql-mongo
generate-graphql-mongo-graphql-schema:
	cargo run -p graphql-mongo --bin graphql-generator

run-graphql-mongo:
	cargo run -p graphql-mongo --bin graphql-mongo

build-graphql-mongo:
	docker build . --target graphql-mongo  -t oyelowo/graphql-mongo

deploy-graphql-mongo:
	docker build -f Dockerfile.development --target graphql-mongo  -t oyelowo/graphql-mongo .
	docker run -p 8000:8000 oyelowo/graphql-mongo 

deploy-graphql-mongo-slim:
	docker build -f Dockerfile.development --target graphql-mongo  -t oyelowo/graphql-mongo .
	docker run --rm -v /var/run/docker.sock:/var/run/docker.sock dslim/docker-slim build  oyelowo/graphql-mongo --tag oyelowo/graphql-mongo.slim
	docker run -p 3000:3000 oyelowo/graphql-mongo.slim


# 2. grpc-mongo
run-grpc-mongo:
	cargo run -p grpc-mongo --bin grpc-mongo

build-grpc-mongo:
	docker build . --target grpc-mongo  -t oyelowo/grpc-mongo

deploy-grpc-mongo:
	docker build -f Dockerfile.development --tar-grpc-mongoget grpc-mongo  -t oyelowo/grpc-mongo .
	docker run -p 3000:3000 oyelowo/grpc-mongo 

deploy-grpc-mongo-slim:
	docker build -f Dockerfile.development --target grpc-mongo  -t oyelowo/grpc-mongo .
	docker run --rm -v /var/run/docker.sock:/var/run/docker.sock dslim/docker-slim build  oyelowo/grpc-mongo --tag oyelowo/grpc-mongo.slim
	docker run -p 3000:3000 oyelowo/grpc-mongo.slim


