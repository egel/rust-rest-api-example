use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::controllers::{health, root, tasks};

/// Builds the application router with a database pool shared across handlers.
pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root::root))
        .route("/health", get(health::health))
        .route("/tasks", get(tasks::tasks_list).post(tasks::task_create))
        .route(
            "/tasks/{id}",
            get(tasks::task_info).delete(tasks::task_delete),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool)
}
