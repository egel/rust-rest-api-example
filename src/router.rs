use axum::{Router, routing::get, http::Request,};
use sqlx::PgPool;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level, Span};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::controllers::{health, root, tasks};
use crate::models::task::{CreateTask, Task};

/// Builds the application router with a database pool shared across handlers.
#[derive(OpenApi)]
#[openapi(
    info(title = "Making API", version = "0.0.1"),
    paths(
        tasks::tasks_list,
        tasks::task_create,
        tasks::task_info,
        tasks::task_delete,
    ),
    components(schemas(Task, CreateTask))
)]
struct ApiDocs;

fn requests_span(request: &Request<Body>) -> Span {
    let request_id = uuid::Uuid::new_v4().to_string();
    tracing::info_span!(
        "HTTP request",
        method = %request.method(),
        path = %request.uri().path(),
        request_id = %request_id,
    );
}

pub fn create_router(pool: PgPool) -> Router {
    let openapi = ApiDocs::openapi();
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
                .make_span_with(requests_span)
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/v1/openapi.json", openapi.clone()))
        .with_state(pool)
}
