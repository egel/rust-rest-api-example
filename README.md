# Rust REST API example

```sh
# copy variables example and adjust
cp .env.example .env

# start infra
docker compose up -d

# build project and dependencies
cargo build

# and run project (in development)
SERVER_ENVIRONMENT=develop cargo run
```

## Development

1.  How to delete database?

    ```sh
    docker compose exec -T database psql -U admin -d postgres \
        -c "DROP DATABASE IF EXISTS rust_rest_api WITH (FORCE);" \
        -c "CREATE DATABASE rust_rest_api;"
    ```

2.  Start server (with migrations)

    ```sh
    # build and run project (in development)
    SERVER_ENVIRONMENT=develop cargo run
    ```

3.  How to fill db with init seed?

    ```sh
    ./scripts/seed.sh
    ```
    
### Testing

Test API locally against local database. Before running init the database with sample seed.

```sh
SERVER_ENVIRONMENT=develop cargo test
```