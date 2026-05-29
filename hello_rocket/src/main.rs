use rocket::serde::json::{Json, Value};
use rocket::{get, launch, routes};

use chrono::{Datelike, Local, Timelike};
use serde_json::json;

#[get("/")]
fn hello() -> &'static str {
    "Hello World"
}

#[get("/sing")]
fn sing() -> &'static str {
    "La la la"
}

#[get("/json")]
fn json_response() -> Json<Value> {
    let now = Local::now();
    Json(json!({
        "year": now.year(),
        "month": now.month(),
        "day": now.day(),
        "hour": now.hour(),
        "minute": now.minute(),
        "second": now.second(),
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, sing, json_response])
}
