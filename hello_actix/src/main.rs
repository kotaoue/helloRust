use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/sing")]
async fn sing() -> impl Responder {
    HttpResponse::Ok().body("La la la")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(sing))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
