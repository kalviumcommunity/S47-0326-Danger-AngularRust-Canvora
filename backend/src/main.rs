mod models;
mod migrations;
mod repository;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse, post, get};
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::web::Data;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use models::*;
use migrations::*;
use repository::*;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
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

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedBoardsResponse {
    pub items: Vec<Board>,
    pub next_cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub repositories: RepositoryFactory,
    pub start_time: SystemTime,
}

fn build_cors() -> Cors {
    let cors_origin = env::var("CORS_ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:4200".to_string());

    Cors::default()
        .allowed_origin(&cors_origin)
        .allowed_methods(["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .expose_headers([header::LINK])
        .supports_credentials()
        .max_age(3600)
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
    let repo = state.repositories.draw_segment_repository();
    let drawings = repo.find_all().await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    let drawings: Vec<DrawSegment> = drawings.into_iter().map(|d| d.into()).collect();
    Ok(HttpResponse::Ok().json(drawings))
}

#[post("/draw")]
async fn add_draw(state: Data<AppState>, item: web::Json<DrawSegment>) -> Result<impl Responder, AppError> {
    let repo = state.repositories.draw_segment_repository();
    let db_segment: DbDrawSegment = item.into_inner().into();

    repo.save(db_segment).await
        .map_err(|e| AppError::InternalError(format!("Failed to save drawing: {}", e)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"success": true})))
}

#[post("/draw/batch")]
async fn add_draw_batch(state: Data<AppState>, items: web::Json<Vec<DrawSegment>>) -> Result<impl Responder, AppError> {
    if items.len() > 1000 {
        return Err(AppError::ValidationError("Batch size too large (max 1000 segments)".to_string()));
    }

    let repo = state.repositories.draw_segment_repository();
    let db_segments: Vec<DbDrawSegment> = items.into_inner().into_iter().map(|s| s.into()).collect();

    let saved_segments = repo.save_batch(db_segments).await
        .map_err(|e| AppError::InternalError(format!("Failed to save drawing batch: {}", e)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "added_count": saved_segments.len()
    })))
}

#[get("/boards")]
async fn get_boards(state: Data<AppState>, query: web::Query<PaginationParams>) -> Result<impl Responder, AppError> {
    let cursor_time = match &query.cursor {
        Some(value) => Some(DateTime::parse_from_rfc3339(value)
            .map_err(|_| AppError::ValidationError("Invalid cursor format".to_string()))?
            .with_timezone(&Utc)),
        None => None,
    };

    let limit = query.limit.unwrap_or(25).clamp(1, 100) as i64;
    let repo = state.repositories.board_repository();
    let boards = repo.find_page(cursor_time, limit).await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    let next_cursor = boards.last().map(|board| board.created_at.to_rfc3339());
    let items: Vec<Board> = boards.into_iter().map(|board| board.into()).collect();

    Ok(HttpResponse::Ok().json(PaginatedBoardsResponse {
        items,
        next_cursor,
        limit: limit as u32,
    }))
}

#[post("/boards")]
async fn create_board(state: Data<AppState>, req: web::Json<CreateBoardRequest>) -> Result<impl Responder, AppError> {
    req.validate().map_err(AppError::ValidationError)?;

    let repo = state.repositories.board_repository();
    let board = Board::new(req.name.clone(), "user-1".to_string(), req.is_public);
    let db_board: DbBoard = board.clone().into();

    let saved_board = repo.save(db_board).await
        .map_err(|e| AppError::InternalError(format!("Failed to create board: {}", e)))?;

    Ok(HttpResponse::Created().json(Board::from(saved_board)))
}

#[get("/boards/{id}")]
async fn get_board(state: Data<AppState>, path: web::Path<String>) -> Result<impl Responder, AppError> {
    let board_id = path.into_inner();
    let uuid = sqlx::types::Uuid::parse_str(&board_id)
        .map_err(|_| AppError::ValidationError("Invalid board ID format".to_string()))?;

    let repo = state.repositories.board_repository();
    let board = repo.find_by_id(uuid).await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    match board {
        Some(b) => Ok(HttpResponse::Ok().json(Board::from(b))),
        None => Err(AppError::NotFound(format!("Board with id '{}' not found", board_id))),
    }
}

#[get("/boards/{id}/drawings")]
async fn get_board_drawings(state: Data<AppState>, path: web::Path<String>) -> Result<impl Responder, AppError> {
    let board_id = path.into_inner();
    let uuid = sqlx::types::Uuid::parse_str(&board_id)
        .map_err(|_| AppError::ValidationError("Invalid board ID format".to_string()))?;

    let repo = state.repositories.draw_segment_repository();
    let drawings = repo.find_by_board(uuid).await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    let drawings: Vec<DrawSegment> = drawings.into_iter().map(|d| d.into()).collect();
    Ok(HttpResponse::Ok().json(drawings))
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

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/whiteboard".to_string());

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to connect to database: {}", e)))?;

    run_migrations(&db)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to run migrations: {}", e)))?;

    let app_state = Data::new(AppState {
        db: db.clone(),
        repositories: RepositoryFactory::new(db),
        start_time: SystemTime::now(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(DefaultHeaders::new()
                .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                .add((header::X_FRAME_OPTIONS, "DENY"))
                .add((header::X_XSS_PROTECTION, "1; mode=block"))
                .add((header::REFERRER_POLICY, "same-origin")))
            .wrap(build_cors())
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
