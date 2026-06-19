use chrono::{Datelike, Local, Timelike};
use serde::Deserialize;
use serde_json::json;
use warp::Filter;

const HOST: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3030;

#[derive(Deserialize)]
struct JsonQuery {
    message: Option<String>,
}

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "Hello World");

    let sing = warp::path("sing").map(|| "La la la");

    let laugh = warp::path("laugh").map(|| "Ha ha ha");

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

    let routes = hello.or(sing).or(laugh).or(json);

    println!("Listening on http://{}.{}.{}.{}:{}", HOST[0], HOST[1], HOST[2], HOST[3], PORT);
    warp::serve(routes).run((HOST, PORT)).await;
}
