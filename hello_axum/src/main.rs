use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};
use chrono::{Datelike, Local, Timelike};
use serde::Deserialize;
use serde_json::json;

async fn hello() -> &'static str {
    "Hello World"
}

async fn hello_plain() -> &'static str {
    "HelloWorld"
}

async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn sing() -> &'static str {
    "La la la"
}

async fn laugh() -> &'static str {
    "Ha ha ha"
}

async fn fail() -> Response {
    let body = Json(json!({
        "error": "intentional failure",
        "status": StatusCode::BAD_REQUEST.as_u16(),
    }));
    (StatusCode::BAD_REQUEST, body).into_response()
}

#[derive(Deserialize)]
struct JsonQuery {
    message: Option<String>,
}

async fn json_handler(Query(params): Query<JsonQuery>) -> Json<serde_json::Value> {
    let now = Local::now();
    let message = params
        .message
        .unwrap_or_else(|| "warp っぽい JSON レスポンス".to_string());
    Json(json!({
        "year": now.year(),
        "month": now.month(),
        "day": now.day(),
        "hour": now.hour(),
        "minute": now.minute(),
        "second": now.second(),
        "message": message,
        "route": "json",
    }))
}

async fn not_found() -> Response {
    let body = Json(json!({
        "error": "route not found",
        "status": StatusCode::NOT_FOUND.as_u16(),
    }));
    (StatusCode::NOT_FOUND, body).into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/hello", get(hello_plain))
        .route("/hello/{name}", get(hello_name))
        .route("/sing", get(sing))
        .route("/laugh", get(laugh))
        .route("/fail", get(fail))
        .route("/json", get(json_handler))
        .fallback(not_found);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
