use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
struct Task {
    id: String,
    name: String,
    done: bool,
    created_at: String,
    updated_at: String,
    deleted_at: String,
}

#[derive(Deserialize)]
struct TaskCreate {
    name: String,
}
