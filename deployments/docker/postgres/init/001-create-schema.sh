#!/bin/sh
# sdkwork-im standalone compose (deployments/docker/docker-compose.yml)
# Creates the canonical workspace PostgreSQL schema used by the container.
# The postgres image only creates the database from POSTGRES_DB; the
# sdkwork-database lifecycle pins search_path to the same-named schema, so
# the schema must exist before the gateway applies migrations. The schema
# name follows the database name (SDKWORK_IM_POSTGRES_DB); keep
# SDKWORK_IM_POSTGRES_SCHEMA equal to it or create the schema manually.
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE SCHEMA IF NOT EXISTS "$POSTGRES_DB";
	GRANT ALL ON SCHEMA "$POSTGRES_DB" TO "$POSTGRES_USER";
EOSQL
