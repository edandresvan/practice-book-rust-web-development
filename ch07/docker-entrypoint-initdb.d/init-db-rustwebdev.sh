#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
  CREATE DATABASE rustwebdev;
  CREATE USER firstdev WITH PASSWORD 'mypassword';
  ALTER DATABASE rustwebdev OWNER TO firstdev;
EOSQL