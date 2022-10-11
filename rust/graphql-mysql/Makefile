update_sqlx_data:
	# chmod +x ./update-sqlx-data.sh
	./update-sqlx-data.sh

migrate:
	sqlx migrate add

migrate_clean:
	docker-compose down db, graphql-mysql
	#docker-compose up --build  graphql-mysql
	# docker-compose up --build graphql-mysql

	# sqlx migrate run

build_db:
	docker create -f Dockerfile.migrations -ti --name dummy IMAGE_NAME bash
	docker cp dummy:/usr/migrations/ ./sql
	docker rm -rf dummy