#[macro_use]
extern crate rocket;

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
fn json_response() -> String {
    let now = Local::now();
    let response = json!({
        "year": now.year(),
        "month": now.month(),
        "day": now.day(),
        "hour": now.hour(),
        "minute": now.minute(),
        "second": now.second(),
    });
    response.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, sing, json_response])
}
