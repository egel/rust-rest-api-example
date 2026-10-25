#!/usr/bin/env bash
set -euo pipefail

# Seeds the local PostgreSQL database by running a SQL file inside the
# database container (psql is not installed on the host).
#
# Usage:
#   ./scripts/seed.sh                        # default: migrations_testseed/init_seed.sql
#   ./scripts/seed.sh path/to/file.sql       # seed with another SQL file

cd "$(dirname "$0")/.."

FILE="${1:-migrations_testseed/init_seed.sql}"

if [[ ! -f "$FILE" ]]; then
    echo "error: SQL file not found: $FILE" >&2
    exit 1
fi

# Load POSTGRES_USER / POSTGRES_DB from .env.
set -a
source .env
set +a
: "${POSTGRES_USER:?POSTGRES_USER missing in .env}"
: "${POSTGRES_DB:?POSTGRES_DB missing in .env}"

CONTAINER="$(docker compose ps -q database)"
if [[ -z "$CONTAINER" ]]; then
    echo "error: database container is not running — start it with: docker compose up -d database" >&2
    exit 1
fi

echo "seeding $POSTGRES_DB (user: $POSTGRES_USER) with $FILE ..."
docker exec -i "$CONTAINER" psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -v ON_ERROR_STOP=1 < "$FILE"
echo "done."
