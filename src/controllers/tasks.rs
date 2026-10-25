use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use sqlx::PgPool;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::models::task::{CreateTask, Task};
use crate::repositories;

/// Type alias for handler results carrying an HTTP error status.
type ApiResult<T> = Result<T, (StatusCode, &'static str)>;

/// `GET /tasks?done=<bool>` - lists tasks, optionally filtered by the `done` flag.
pub async fn tasks_list(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> ApiResult<Json<Vec<Task>>> {
    let done = match params.get("done") {
        Some(value) => match value.parse::<bool>() {
            Ok(done) => Some(done),
            Err(_) => {
                warn!(filter = value, "invalid `done` filter ignored");
                None
            }
        },
        None => None,
    };

    match repositories::tasks::list(&pool, done).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(source) => {
            error!(%source, "failed to list tasks");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))
        }
    }
}

/// `POST /tasks` - creates a task from the JSON payload.
pub async fn task_create(
    State(pool): State<PgPool>,
    Json(CreateTask { name }): Json<CreateTask>,
) -> ApiResult<(StatusCode, Json<Task>)> {
    match repositories::tasks::create(&pool, &name).await {
        Ok(task) => {
            info!(task_id = %task.id, "task created");
            Ok((StatusCode::CREATED, Json(task)))
        }
        Err(source) => {
            error!(%source, "failed to create task");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))
        }
    }
}

/// `GET /tasks/{id}` - returns a single task, or a 404 when missing.
pub async fn task_info(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> ApiResult<Json<Task>> {
    match repositories::tasks::find_by_id(&pool, id).await {
        Ok(Some(task)) => Ok(Json(task)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "task not found")),
        Err(source) => {
            error!(%source, "failed to find task");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))
        }
    }
}

/// `DELETE /tasks/{id}` - soft-deletes a task, returning the archived row.
pub async fn task_delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Task>> {
    match repositories::tasks::delete(&pool, id).await {
        Ok(Some(task)) => {
            info!(task_id = %task.id, "task deleted");
            Ok(Json(task))
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, "task not found")),
        Err(source) => {
            error!(%source, "failed to delete task");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))
        }
    }
}
