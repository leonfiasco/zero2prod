#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null
then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "    cargo install sqlx-cli --no-default-features --features rustls,postgres --locked"
    echo >&2 "to install it."
    exit 1
fi

# Configuration
DB_PORT="${POSTGRES_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
APP_USER="${APP_USER:=app}"
APP_USER_PWD="${APP_USER_PWD:=passwordca}"
APP_DB_NAME="${APP_DB_NAME:=newsletter}"
CONTAINER_NAME="postgres"

# Only start Docker if SKIP_DOCKER is not set
if [[ -z "${SKIP_DOCKER}" || "${SKIP_DOCKER}" == "false" ]]; then
    # Stop and remove any existing container
    if docker ps -a --filter "name=${CONTAINER_NAME}" | grep -q "${CONTAINER_NAME}"; then
        docker rm -f "${CONTAINER_NAME}"
    fi

    # Start new PostgreSQL container
    docker run \
        --env POSTGRES_USER=${SUPERUSER} \
        --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
        --health-cmd="pg_isready -U ${SUPERUSER}" \
        --health-interval=1s \
        --health-timeout=5s \
        --health-retries=5 \
        --publish "${DB_PORT}":5432 \
        --detach \
        --name "${CONTAINER_NAME}" \
        postgres -N 1000

    # Wait for PostgreSQL to be ready
    until [ "$(docker inspect -f '{{.State.Health.Status}}' ${CONTAINER_NAME})" = "healthy" ]; do
        >&2 echo "Postgres is still unavailable - sleeping"
        sleep 1
    done
    >&2 echo "Postgres is up and running on port ${DB_PORT}!"
fi

# Create application user and database
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "CREATE DATABASE ${APP_DB_NAME} OWNER ${APP_USER};"
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "GRANT ALL PRIVILEGES ON DATABASE ${APP_DB_NAME} TO ${APP_USER};"
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "ALTER USER ${APP_USER} CREATEDB;"

# Set DATABASE_URL and run migrations
export DATABASE_URL="postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}"
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been configured:"
>&2 echo "- Superuser: ${SUPERUSER}"
>&2 echo "- Application database: ${APP_DB_NAME}"
>&2 echo "- Application user: ${APP_USER}"