//! Application logging based on the `tracing` ecosystem, configured per
//! [`envs::ENVIRONMENT_OPTIONS`].
//!
//! - `develop`: human-readable, pretty output for the terminal.
//! - Any other environment: JSON lines, ready for log aggregation.
//!
//! Default trace directives also change per environment, but they can always
//! be overridden with the `SERVER_LOG_LEVEL` variable, using the same syntax
//! as `RUST_LOG` (e.g. `SERVER_LOG_LEVEL=info,Rust_REST_API_example=trace`).

use std::env;

use tracing_subscriber::{
    filter::EnvFilter,
    fmt,
    layer::{Layer, SubscriberExt},
    util::SubscriberInitExt,
};

use crate::envs;

/// Name of the environment variable holding trace filter directives, which
/// override the environment-specific defaults from [`default_directives`].
const LOG_LEVEL_KEY: &str = "SERVER_LOG_LEVEL";

/// Name of the environment that uses the pretty terminal formatter.
const DEVELOP_ENVIRONMENT: &str = "develop";

/// Default trace filter directives per application environment.
fn default_directives() -> String {
    match envs::environment().as_str() {
        DEVELOP_ENVIRONMENT => "info,Rust_REST_API_example=debug",
        _ => "info",
    }
    .to_string()
}

/// Builds the trace filter, reading [`LOG_LEVEL_KEY`] and falling back to the
/// environment-specific defaults from [`default_directives`]. Invalid or
/// unknown directives are ignored.
fn filter() -> EnvFilter {
    match env::var(LOG_LEVEL_KEY).ok() {
        Some(directives) => EnvFilter::new(directives),
        None => EnvFilter::new(default_directives()),
    }
}

/// Initializes the global default tracing subscriber. Must be called exactly
/// once, early in `main` (after the `.env` file is loaded, since the
/// environment name controls the output format and default level).
pub fn init_logging() {
    let formatter = if envs::environment() != DEVELOP_ENVIRONMENT {
        fmt::layer()
            .json()
            .with_file(true)
            .with_line_number(true)
            .boxed()
    } else {
        fmt::layer().pretty().boxed()
    };

    tracing_subscriber::registry()
        .with(filter())
        .with(formatter)
        .init();
}
