use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::{SystemTime, UNIX_EPOCH};

use models::*;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub uptime_seconds: u64,
    pub version: String,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub board: Arc<Mutex<Vec<DrawSegment>>>,
    pub boards: Arc<Mutex<Vec<Board>>>,
    pub users: Arc<Mutex<Vec<User>>>,
    pub start_time: SystemTime,
}

#[get("/health")]
async fn health(state: Data<AppState>) -> impl Responder {
    let now = SystemTime::now();
    let uptime = now.duration_since(state.start_time).unwrap_or_default().as_secs();

    let health = HealthResponse {
        status: "healthy".to_string(),
        uptime_seconds: uptime,
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
    };

    HttpResponse::Ok().json(health)
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

#[get("/boards")]
async fn get_boards(state: Data<AppState>) -> impl Responder {
    let boards = state.boards.lock().unwrap();
    HttpResponse::Ok().json(&*boards)
}

#[post("/boards")]
async fn create_board(state: Data<AppState>, req: web::Json<CreateBoardRequest>) -> impl Responder {
    let mut boards = state.boards.lock().unwrap();
    let board = Board::new(req.name.clone(), "user-1".to_string(), req.is_public);
    boards.push(board.clone());
    HttpResponse::Created().json(board)
}

#[get("/boards/{id}")]
async fn get_board(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let board_id = path.into_inner();
    let boards = state.boards.lock().unwrap();
    let board = boards.iter().find(|b| b.id == board_id);

    match board {
        Some(b) => HttpResponse::Ok().json(b),
        None => HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Board not found".to_string()),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        }),
    }
}

#[get("/boards/{id}/drawings")]
async fn get_board_drawings(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let board_id = path.into_inner();
    let drawings = state.board.lock().unwrap();
    let board_drawings: Vec<&DrawSegment> = drawings.iter().filter(|d| d.board_id == board_id).collect();
    HttpResponse::Ok().json(board_drawings)
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
        boards: Arc::new(Mutex::new(Vec::new())),
        users: Arc::new(Mutex::new(Vec::new())),
        start_time: SystemTime::now(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .service(health)
            .service(get_state)
            .service(add_draw)
            .service(get_boards)
            .service(create_board)
            .service(get_board)
            .service(get_board_drawings)
            .route("/ws", web::get().to(ws_handler))
    })
    .bind(addr)?
    .run()
    .await
}
