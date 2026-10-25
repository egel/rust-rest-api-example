use axum::extract::{Path, Query};
use std::collections::HashMap;
use tracing::info;

pub async fn tasks_list(Query(params): Query<HashMap<String, String>>) -> String {
    let filters = params
        .get("done")
        .map_or("everything".to_string(), |x| format!("done={}", x));

    format!("listing tasks, filter: {}", filters)
}

pub async fn task_create() -> &'static str {
    info!("task created");
    "task created"
}

pub async fn task_info(Path(id): Path<String>) -> &'static str {
    "task info"
}
