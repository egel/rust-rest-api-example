use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, info};
use utoipa::IntoParams;
use uuid::Uuid;

use crate::models::task::{CreateTask, Task};
use crate::repositories;

/// Type alias for handler results carrying an HTTP error status.
type ApiResult<T> = Result<T, (StatusCode, &'static str)>;

const TASK_PATH: &str = "/tasks";

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct TaskListQuery {
    /// Filter tasks by completion status. When omitted, all tasks are returned.
    #[param(example = true)]
    pub is_done: Option<bool>,
}

/// `GET /tasks?done=<bool>` - lists tasks, optionally filtered by the `done` flag.
#[utoipa::path(
  get,
  path = TASK_PATH,
  params(TaskListQuery),
  responses(
    (status = StatusCode::OK, description = "List of all tasks.", body = Vec<Task>),
  ),
  tag = "tasks"
)]
pub async fn tasks_list(
    State(pool): State<PgPool>,
    Query(params): Query<TaskListQuery>,
) -> ApiResult<Json<Vec<Task>>> {
    let done = params.is_done;

    match repositories::tasks::list(&pool, done).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(source) => {
            error!(%source, "failed to list tasks");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "internal server error"))
        }
    }
}

/// `POST /tasks` - creates a task from the JSON payload.
#[utoipa::path(
  post,
  path = TASK_PATH,
  request_body = CreateTask,
  responses(
    (status = StatusCode::CREATED, description = "Task created.", body = Task),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error."),
  ),
  tag = "tasks"
)]
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
#[utoipa::path(
  get,
  path = "/tasks/{id}",
  params(("id" = Uuid, Path, description = "Identifier of the task to fetch.")),
  responses(
    (status = StatusCode::OK, description = "Task found.", body = Task),
    (status = StatusCode::NOT_FOUND, description = "Task not found."),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error."),
  ),
  tag = "tasks"
)]
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
#[utoipa::path(
  delete,
  path = "/tasks/{id}",
  params(("id" = Uuid, Path, description = "Identifier of the task to delete.")),
  responses(
    (status = StatusCode::OK, description = "Task deleted.", body = Task),
    (status = StatusCode::NOT_FOUND, description = "Task not found."),
    (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error."),
  ),
  tag = "tasks"
)]
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
