use chrono::{Datelike, Local, Timelike};
use serde::Deserialize;
use serde_json::json;
use warp::Filter;
use warp::http::StatusCode;

const HOST: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3030;

#[derive(Deserialize)]
struct JsonQuery {
    message: Option<String>,
}

#[derive(Debug)]
struct FailRoute;

impl warp::reject::Reject for FailRoute {}

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "Hello World");

    let hello_plain = warp::path!("hello").map(|| "HelloWorld");

    let hello_name = warp::path!("hello" / String).map(|name: String| {
        format!("Hello, {}!", name)
    });

    let sing = warp::path("sing").map(|| "La la la");

    let laugh = warp::path("laugh").map(|| "Ha ha ha");

    let fail = warp::path("fail")
        .and(warp::get())
        .and_then(|| async move {
            Err::<warp::reply::Response, _>(warp::reject::custom(FailRoute))
        });

    let json = warp::path("json")
        .and(warp::get())
        .and(warp::query::<JsonQuery>())
        .map(|query: JsonQuery| {
            let now = Local::now();
            let response = json!({
                "year": now.year(),
                "month": now.month(),
                "day": now.day(),
                "hour": now.hour(),
                "minute": now.minute(),
                "second": now.second(),
                "message": query
                    .message
                    .unwrap_or_else(|| "warp っぽい JSON レスポンス".to_string()),
                "route": "json",
            });
            warp::reply::with_header(warp::reply::json(&response), "X-Warp-Style", "filters")
        });

    let routes = hello
        .or(hello_plain)
        .or(hello_name)
        .or(sing)
        .or(laugh)
        .or(fail)
        .or(json)
        .recover(|err: warp::Rejection| async move {
            let (status, message) = if err.find::<FailRoute>().is_some() {
                (StatusCode::BAD_REQUEST, "intentional failure")
            } else if err.is_not_found() {
                (StatusCode::NOT_FOUND, "route not found")
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "unhandled warp rejection")
            };

            let body = json!({
                "error": message,
                "status": status.as_u16(),
            });

            Ok::<_, std::convert::Infallible>(warp::reply::with_status(
                warp::reply::json(&body),
                status,
            ))
        });

    println!("Listening on http://{}.{}.{}.{}:{}", HOST[0], HOST[1], HOST[2], HOST[3], PORT);
    warp::serve(routes).run((HOST, PORT)).await;
}
