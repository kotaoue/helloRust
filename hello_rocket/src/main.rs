use rocket::form::FromForm;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use rocket::{catch, catchers, get, launch, routes};

use chrono::{Datelike, Local, Timelike};
use serde_json::json;

#[derive(FromForm)]
struct JsonQuery {
    message: Option<String>,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello World"
}

#[get("/hello")]
fn hello_plain() -> &'static str {
    "HelloWorld"
}

#[get("/hello/<name>")]
fn hello_name(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/sing")]
fn sing() -> &'static str {
    "La la la"
}

#[get("/laugh")]
fn laugh() -> &'static str {
    "Ha ha ha"
}

#[get("/fail")]
fn fail() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::BadRequest,
        Json(json!({
            "error": "intentional failure",
            "status": Status::BadRequest.code,
        })),
    )
}

#[get("/json?<query..>")]
fn json_response(query: Option<JsonQuery>) -> Json<Value> {
    let now = Local::now();
    let message = query
        .and_then(|params| params.message)
        .unwrap_or_else(|| "warp cぽい JSON レスポンス".to_string());

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

#[catch(404)]
fn not_found() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::NotFound,
        Json(json!({
            "error": "route not found",
            "status": Status::NotFound.code,
        })),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![hello, hello_plain, hello_name, sing, laugh, fail, json_response],
        )
        .register("/", catchers![not_found])
}
