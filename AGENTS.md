# AGENTS.md

## Commands

- Build: `cargo build`; Run: `cargo run`
- Verify changes: `cargo fmt --check && cargo clippy && cargo test` (no tests exist yet, but run it anyway)
- Rust edition 2024 — requires a recent stable toolchain (1.85+).

## State of the project

- Single-crate Actix Web 4 API example, still a skeleton: `src/main.rs` only prints a line; no routes, no tests, no CI yet. Expect to wire things up rather than extend them.
- The crate name is `Rust_REST_API_example` (mixed case) — binary/artifact names reflect it.

## Protocol buffers

- `protobuf/task.proto` defines the `Task` message (id, name, timestamps via `google.protobuf.Timestamp`), but codegen is NOT wired: no `prost`/`tonic` deps and no `build.rs` yet. Don't assume generated Rust types exist.
- If adding codegen, follow the standard `build.rs` + `prost-build` setup and generate into `target/` or an OUT_DIR — don't hand-edit generated files.

## Docs

- `docs/assets/overview_excalidraw.svg` is the architecture overview.
