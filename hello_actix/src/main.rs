use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/sing")]
async fn sing() -> impl Responder {
    HttpResponse::Ok().body("La la la")
}

#[get("/json")]
async fn json() -> impl Responder {
    let now = Local::now();
    let response = json!({
        "year": now.year(),
        "month": now.month(),
        "day": now.day(),
        "hour": now.hour(),
        "minute": now.minute(),
        "second": now.second(),
    });
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
    .service(hello)
    .service(sing)
    .service(json))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
