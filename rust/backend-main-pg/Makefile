migrate:
	sqlx migrate add

build_db:
	docker create -f Dockerfile.migrations -ti --name dummy IMAGE_NAME bash
	docker cp dummy:/usr/migrations/ ./sql
	docker rm -rf dummy