use chrono::{Datelike, Local, Timelike};
use serde_json::json;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "Hello World");

    let sing = warp::path("sing").map(|| "La la la");

    let json = warp::path("json").map(|| {
        let now = Local::now();
        let response = json!({
            "year": now.year(),
            "month": now.month(),
            "day": now.day(),
            "hour": now.hour(),
            "minute": now.minute(),
            "second": now.second(),
        });
        warp::reply::json(&response)
    });

    let routes = hello.or(sing).or(json);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
