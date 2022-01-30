migrate:
	sqlx migrate add

migrate_clean:
	docker-compose down db, backend-main-pg
	#docker-compose up --build  backend-main-pg
	# docker-compose up --build backend-main-pg

	# sqlx migrate run

build_db:
	docker create -f Dockerfile.migrations -ti --name dummy IMAGE_NAME bash
	docker cp dummy:/usr/migrations/ ./sql
	docker rm -rf dummy