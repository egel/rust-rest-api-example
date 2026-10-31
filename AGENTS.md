# AGENTS.md

## Commands

- Build: `cargo build`; Run: `cargo run`
- Verify changes: `cargo fmt --check && cargo clippy && cargo test`
- Run API tests only: `cargo test --test api`
- Tests need env vars exported first (`set -a; source .env; set +a`): `DATABASE_URL` is required at build time by the compile-time checked queries, `SERVER_DATABASE_URL` is used by the tests. The test Postgres must already have migrations applied.
- Rust edition 2024 — requires a recent stable toolchain (1.85+).

## Configuration & running

- Environment variables (all required): `SERVER_DATABASE_URL`, `SERVER_ENVIRONMENT`, `SERVER_HOST`, `SERVER_PORT`.
- `SERVER_ENVIRONMENT` must be one of `develop`, `production`, `testing`. A `.env` file is loaded only when the environment is `develop`; for other environments provide real variables.
- A reachable Postgres is required at startup: refinery migrations apply automatically, then sqlx connects the pool.
- SQL is compile-time checked (`sqlx::query_as!`): building the crate (including tests) needs a reachable `DATABASE_URL` or offline `.sqlx` metadata via `cargo sqlx prepare`.

## State of the project

- Single-crate Axum 0.8 (not Actix) REST API for tasks, wired and running: controllers (HTTP handlers) → repositories (SQL) → models (`src/models/task.rs`).
- Routes (`src/router.rs`): `GET /`, `GET /health`, `GET+POST /tasks`, `GET+DELETE /tasks/{id}`. Tasks support an `is_done` filter param; deletes are soft (stamped `deleted_at`, excluded from queries).
- Migrations via refinery, embedded from `./migrations` (refinery naming, e.g. `V1__init.sql`).
- Observability: `tower-http` TraceLayer, `tracing-subscriber` (JSON).
- Lib/bin split is done: `src/lib.rs` exposes the modules as `pub mod`; `src/main.rs` is a thin entry point calling into the library. Integration tests under `tests/` import via `rust_rest_api_example::…`.
- API tests live in `tests/api/` (directory-style test target `api`, `tests/api/main.rs` + one file per feature, declared as `mod`s in `main.rs`; `harness = true` is set explicitly in Cargo.toml). Unit tests: inline `#[cfg(test)] mod tests` in the source files. No CI yet.
- The crate name is `rust_rest_api_example` (snake case) — binary/artifact names reflect it.

## Protocol buffers

- `proto/api/v1/task.proto` defines the `Task` message (id, name, timestamps via `google.protobuf.Timestamp`), but codegen is NOT wired and no generated code is used. Caveats: provisioning for it exists (`tonic`/`prost`/`tonic-prost-build` dependencies, a `[build-dependencies]` entry) and a script was written to `src/build.rs` — wrong location (Cargo expects `build.rs` at the package root or `build = "src/build.rs"` in Cargo.toml) and currently declared as an unused `mod build` in `main.rs`, so it never runs. It also calls an older `.compile_protos` API; expect to fix it, not trust it.
- If wiring codegen: move the script to `build.rs`, use the current `tonic-prost-build`/`prost-build` API, generate into `OUT_DIR`/`target` — don't hand-edit generated files.

## Docs

- `docs/assets/overview_excalidraw.svg` is the architecture overview.
