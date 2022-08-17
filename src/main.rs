use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
use actix_web::{middleware::Logger};
use actix_cors::Cors;

#[derive(Serialize)]
struct User {
    name: String
}

#[get("/")]
async fn hello() -> Result<impl Responder> {
    let obj = User {
        name: "John".to_string()
    };
    Ok(web::Json(obj))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!11111")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600)
        )
        .wrap(Logger::default())
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}