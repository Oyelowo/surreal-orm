format:
	cargo fmt
	cargo clippy -fix

check:
	cargo clippy

clippy-fix:
	cargo clippy --fix

# backend-main
run-backend-main:
	cargo run -p backend-main --bin backend-main

build-backend-main:
	docker build . --target backend-main  -t oyelowo/backend-main

deploy-backend-main:
	docker build -f Dockerfile.prod --tar-backend-mainget backend-main  -t oyelowo/backend-main .
	docker run -p 3000:3000 oyelowo/backend-main 


# backend-second
run-backend-second:
	cargo run -p backend-second --bin backend-second

build-backend-second:
	docker build . --target backend-second  -t oyelowo/backend-second

deploy-backend-second:
	docker build -f Dockerfile.prod --tar-backend-secondget backend-second  -t oyelowo/backend-second .
	docker run -p 3000:3000 oyelowo/backend-second 

deploy-web:
	docker build -f Dockerfile.prod --target web  -t oyelowo/web .
	docker-slim build --include-path=/app/apps/web/server.js oyelowo/web
	docker run -p 3000:3000 oyelowo/web.slim


