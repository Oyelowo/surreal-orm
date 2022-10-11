#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.6.2 sqlx-cli --no-default-features --features mysql"
  echo >&2 "to install it."
  exit 1
fi

DB_USER=${MYSQL_USER:=mysql}
DB_PASSWORD="${MYSQL_PASSWORD:=1234}"
DB_ROOT_PASSWORD="${MYSQL_ROOT_PASSWORD:=1234}"
DB_NAME="${MYSQL_DB:=mydb}"
DB_PORT="${MYSQL_PORT:=14000}"

# Allow to skip Docker if a dockerized MySql database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker stop "$(docker ps | grep "${DB_PORT}" | awk '{print $1}')"
  docker run \
      -e MYSQL_USER=${DB_USER} \
      -e MYSQL_ROOT_PASSWORD=${DB_PASSWORD} \
      -e MYSQL_PASSWORD=${DB_PASSWORD} \
      -e MYSQL_DATABASE=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d mysql \
      mysql -N 1000
fi
       - MYSQL_ROOT_PASSWORD=abcd
          - MYSQL_PASSWORD=abcd
          - MYSQL_DATABASE=abcdDB
          # mysql --comments -h 127.0.0.1 -P 14000 -u root

until MYSQL_PASSWORD="${DB_PASSWORD}" mysql --comments -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -c '\q'; do
  >&2 echo "Mysql is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Mysql is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run
# cargo sqlx prepare --check
cargo sqlx prepare

>&2 echo "Mysql has been migrated, ready to go!"