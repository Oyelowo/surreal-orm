fmt:
	SQLX_OFFLINE=true cargo fmt --all -- --check
	# SQLX_OFFLINE=true cargo clippy -fix

clippy:
	SQLX_OFFLINE=true cargo clippy -- -D  warnings

clippy-fix:
	SQLX_OFFLINE=true cargo clippy --fix

test:
	SQLX_OFFLINE=true cargo test

# 1. backend-main
run-backend-main:
	cargo run -p backend-main --bin backend-main

build-backend-main:
	docker build . --target backend-main  -t oyelowo/backend-main

deploy-backend-main:
	docker build -f Dockerfile.prod --target backend-main  -t oyelowo/backend-main .
	docker run -p 8000:8000 oyelowo/backend-main 

deploy-backend-main-slim:
	docker build -f Dockerfile.prod --target backend-main  -t oyelowo/backend-main .
	docker run --rm -v /var/run/docker.sock:/var/run/docker.sock dslim/docker-slim build  oyelowo/backend-main --tag oyelowo/backend-main.slim
	docker run -p 3000:3000 oyelowo/backend-main.slim


# 2. backend-second
run-backend-second:
	cargo run -p backend-second --bin backend-second

build-backend-second:
	docker build . --target backend-second  -t oyelowo/backend-second

deploy-backend-second:
	docker build -f Dockerfile.prod --tar-backend-secondget backend-second  -t oyelowo/backend-second .
	docker run -p 3000:3000 oyelowo/backend-second 

deploy-backend-second-slim:
	docker build -f Dockerfile.prod --target backend-second  -t oyelowo/backend-second .
	docker run --rm -v /var/run/docker.sock:/var/run/docker.sock dslim/docker-slim build  oyelowo/backend-second --tag oyelowo/backend-second.slim
	docker run -p 3000:3000 oyelowo/backend-second.slim


