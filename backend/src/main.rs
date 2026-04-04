use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get, Error as ActixError};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::http::StatusCode;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

use models::*;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    InternalError(String),
    LockError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::LockError(msg) => write!(f, "Lock error: {}", msg),
        }
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(self.to_string()),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        };

        match self {
            AppError::NotFound(_) => HttpResponse::NotFound().json(error_response),
            AppError::ValidationError(_) => HttpResponse::BadRequest().json(error_response),
            AppError::InternalError(_) => HttpResponse::InternalServerError().json(error_response),
            AppError::LockError(_) => HttpResponse::InternalServerError().json(error_response),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub uptime_seconds: u64,
    pub version: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBoardRequest {
    pub name: String,
    pub is_public: bool,
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
async fn get_state(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let board = state.board.lock().map_err(|_| AppError::LockError("Failed to acquire drawings lock".to_string()))?;
    Ok(HttpResponse::Ok().json(&*board))
}

#[post("/draw")]
async fn add_draw(state: Data<AppState>, item: web::Json<DrawSegment>) -> Result<impl Responder, AppError> {
    let mut board = state.board.lock().map_err(|_| AppError::LockError("Failed to acquire drawings lock".to_string()))?;
    board.push(item.into_inner());
    Ok(HttpResponse::Ok().json(&*board))
}

#[post("/draw/batch")]
async fn add_draw_batch(state: Data<AppState>, items: web::Json<Vec<DrawSegment>>) -> Result<impl Responder, AppError> {
    // Validate batch size
    if items.len() > 1000 {
        return Err(AppError::ValidationError("Batch size too large (max 1000 segments)".to_string()));
    }

    let mut board = state.board.lock().map_err(|_| AppError::LockError("Failed to acquire drawings lock".to_string()))?;
    let mut added_count = 0;
    for item in items.into_inner() {
        board.push(item);
        added_count += 1;
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "added_count": added_count,
        "total_segments": board.len()
    })))
}

#[post("/draw/batch")]
async fn add_draw_batch(state: Data<AppState>, items: web::Json<Vec<DrawSegment>>) -> impl Responder {
    let mut board = state.board.lock().unwrap();
    let mut added_count = 0;
    for item in items.into_inner() {
        board.push(item);
        added_count += 1;
    }
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "added_count": added_count,
        "total_segments": board.len()
    }))
}

#[get("/boards")]
async fn get_boards(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let boards = state.boards.lock().map_err(|_| AppError::LockError("Failed to acquire boards lock".to_string()))?;
    Ok(HttpResponse::Ok().json(&*boards))
}

#[post("/boards")]
async fn create_board(state: Data<AppState>, req: web::Json<CreateBoardRequest>) -> Result<impl Responder, AppError> {
    // Validate input
    req.validate().map_err(AppError::ValidationError)?;

    // Acquire lock with error handling
    let mut boards = state.boards.lock().map_err(|_| AppError::LockError("Failed to acquire boards lock".to_string()))?;

    // Create and store board
    let board = Board::new(req.name.clone(), "user-1".to_string(), req.is_public);
    boards.push(board.clone());

    Ok(HttpResponse::Created().json(board))
}

#[get("/boards/{id}")]
async fn get_board(state: Data<AppState>, path: web::Path<String>) -> Result<impl Responder, AppError> {
    let board_id = path.into_inner();

    // Acquire lock with error handling
    let boards = state.boards.lock().map_err(|_| AppError::LockError("Failed to acquire boards lock".to_string()))?;

    // Find board
    let board = boards.iter().find(|b| b.id == board_id);

    match board {
        Some(b) => Ok(HttpResponse::Ok().json(b)),
        None => Err(AppError::NotFound(format!("Board with id '{}' not found", board_id))),
    }
}

#[get("/boards/{id}/drawings")]
async fn get_board_drawings(state: Data<AppState>, path: web::Path<String>) -> Result<impl Responder, AppError> {
    let board_id = path.into_inner();

    // Acquire lock with error handling
    let drawings = state.board.lock().map_err(|_| AppError::LockError("Failed to acquire drawings lock".to_string()))?;

    // Filter drawings for the board
    let board_drawings: Vec<&DrawSegment> = drawings.iter().filter(|d| d.board_id == board_id).collect();
    Ok(HttpResponse::Ok().json(board_drawings))
}

async fn ws_handler() -> impl Responder {
    HttpResponse::Ok().body("WebSocket endpoint - TODO")
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
            .service(add_draw_batch)
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
