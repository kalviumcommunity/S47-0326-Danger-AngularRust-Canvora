use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::get;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;
use std::env;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

// WebSocket handler placeholder
async fn ws_handler() -> impl Responder {
    HttpResponse::Ok().body("WebSocket endpoint")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    println!("Starting server at http://{}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(health)
            .route("/ws", web::get().to(ws_handler))
    })
    .bind(addr)?
    .run()
    .await
}
