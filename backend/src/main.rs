use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::get;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DrawPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DrawSegment {
    pub id: String,
    pub user_id: String,
    pub points: Vec<DrawPoint>,
    pub color: String,
    pub width: f32,
}

#[derive(Clone)]
pub struct AppState {
    pub board: Arc<Mutex<Vec<DrawSegment>>>,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/state")]
async fn get_state(state: Data<AppState>) -> impl Responder {
    let board = state.board.lock().unwrap();
    HttpResponse::Ok().json(&*board)
}

#[post("/draw")]
async fn add_draw(state: Data<AppState>, item: web::Json<DrawSegment>) -> impl Responder {
    let mut board = state.board.lock().unwrap();
    board.push(item.into_inner());
    HttpResponse::Ok().json(&*board)
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

    let app_state = Data::new(AppState {
        board: Arc::new(Mutex::new(Vec::new())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .service(health)
            .service(get_state)
            .service(add_draw)
            .route("/ws", web::get().to(ws_handler))
    })
    .bind(addr)?
    .run()
    .await
}
