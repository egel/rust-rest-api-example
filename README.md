# Rust REST API example

```sh
# copy variables example and adjust
cp .env.example .env

# start infra
docker compose up -d

# build and run project (in development)
SERVER_ENVIRONMENT=develop cargo run
```

## Development

How to fill db with init seed?

```sh
./scripts/seed.sh
```

How to delete database?

```sh
docker compose exec -T database psql -U admin -d postgres \
  -c "DROP DATABASE rust_rest_api WITH (FORCE);" \
  -c "CREATE DATABASE rust_rest_api;"
```
