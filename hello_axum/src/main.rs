use axum::{routing::get, Router};
use chrono::{Datelike, Local, Timelike};
use serde_json::json;

async fn hello() -> &'static str {
    "Hello World"
}

async fn sing() -> &'static str {
    "La la la"
}

async fn json_handler() -> axum::Json<serde_json::Value> {
    let now = Local::now();
    let response = json!({
        "year": now.year(),
        "month": now.month(),
        "day": now.day(),
        "hour": now.hour(),
        "minute": now.minute(),
        "second": now.second(),
    });
    axum::Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/sing", get(sing))
        .route("/json", get(json_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
