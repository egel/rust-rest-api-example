//! End-to-end tests for the `/tasks` endpoints.
//!
//! These tests need a reachable Postgres: `SERVER_DATABASE_URL` must point
//! to a database with migrations already applied (see `scripts/seed.sh` /
//! the app itself, which runs refinery migrations at startup).

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

use rust_rest_api_example::{db, envs, router};

const BODY_LIMIT: usize = 1_048_576;

const TASKS_PATH: &str = "/tasks";

async fn test_app() -> axum::Router {
    envs::load_dotenv();
    let pool = db::connect(envs::database_url()).await.unwrap();
    router::create_router(pool)
}

async fn body_json(res: axum::http::Response<Body>) -> Value {
    serde_json::from_slice(&to_bytes(res.into_body(), BODY_LIMIT).await.unwrap())
        .expect("response body is not valid JSON")
}

#[tokio::test]
async fn task_round_trip() {
    let app = test_app().await;

    // POST /tasks -> 201 + created task
    let res = app
        .clone()
        .oneshot(
            Request::post(TASKS_PATH)
                .header("content-type", "application/json")
                .body(Body::from(json!({"name": "write tests"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let task = body_json(res).await;
    let id = task["id"].as_str().unwrap();
    assert_eq!(task["name"], "write tests");
    assert_eq!(task["is_done"], json!(false));
    assert_eq!(task["deleted_at"], json!(null));

    // GET /tasks/{id} -> 200 with the same task
    let res = app
        .clone()
        .oneshot(
            Request::get(format!("{TASKS_PATH}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let fetched = body_json(res).await;
    assert_eq!(fetched["id"], task["id"]);

    // DELETE /tasks/{id} -> 200 with the archived (soft-deleted) row
    let res = app
        .clone()
        .oneshot(
            Request::delete(format!("{TASKS_PATH}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let archived = body_json(res).await;
    assert_eq!(archived["id"], task["id"]);
    assert!(archived["deleted_at"].is_string());

    // After the soft delete, both GET and DELETE -> 404
    let res = app
        .clone()
        .oneshot(
            Request::get(format!("{TASKS_PATH}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let res = app
        .oneshot(
            Request::delete(format!("{TASKS_PATH}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_missing_task_is_404() {
    let app = test_app().await;

    // A valid, never-created UUID reaches the handler and yields 404.
    let id = Uuid::new_v4();
    let res = app
        .clone()
        .oneshot(
            Request::get(format!("{TASKS_PATH}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_task_with_malformed_id_is_400() {
    let app = test_app().await;

    // Not a UUID: the path extractor rejects it before any DB lookup,
    // so this exercises axum's rejection path rather than the handler.
    let res = app
        .oneshot(
            Request::get(format!("{TASKS_PATH}/not-a-uuid"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
