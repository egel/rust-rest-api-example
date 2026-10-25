use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A task entity, matching the `tasks` database table.
#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub is_done: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Incoming payload accepted when creating a task.
#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub name: String,
}
