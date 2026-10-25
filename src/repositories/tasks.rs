//! SQL queries for the `tasks` table.

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::task::Task;

/// Lists all tasks (excluding soft-deleted ones), optionally filtered by the
/// `done` flag.
pub async fn list(pool: &PgPool, done: Option<bool>) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        "SELECT id, name, done, created_at, updated_at, deleted_at
        FROM tasks
        WHERE deleted_at IS NULL AND ($1::boolean IS NULL OR done = $1)
        ORDER BY created_at, updated_at",
        done
    )
    .fetch_all(pool)
    .await
}

/// Creates a task with the given name and returns the stored row.
pub async fn create(pool: &PgPool, name: &str) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        "INSERT INTO tasks (name) VALUES ($1)
        RETURNING id, name, done, created_at, updated_at, deleted_at",
        name
    )
    .fetch_one(pool)
    .await
}

/// Soft-deletes a task by stamping `deleted_at`, excluding already-deleted
/// rows, and returns the archived row so the caller knows which one was hit.
/// Returns `None` when the task does not exist (it never soft-deletes hard rows).
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        "UPDATE tasks SET deleted_at = now()
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, name, done, created_at, updated_at, deleted_at",
        id
    )
    .fetch_optional(pool)
    .await
}

/// Finds a single task (excluding soft-deleted ones) by its id.
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        "SELECT id, name, done, created_at, updated_at, deleted_at
        FROM tasks
        WHERE id = $1 AND deleted_at IS NULL",
        id
    )
    .fetch_optional(pool)
    .await
}
