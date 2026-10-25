use axum::{Router, routing::get};

use crate::controllers;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(controllers::root))
        .route("/health", get(controllers::health))
        .route(
            "/tasks",
            get(controllers::tasks_list).post(controllers::task_create),
        )
        .route("/tasks/{id}", get(controllers::task_info))
}
