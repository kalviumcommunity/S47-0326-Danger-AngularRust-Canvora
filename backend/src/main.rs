mod models;
mod migrations;
mod repository;

use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get, Error as ActixError};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::http::StatusCode;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use models::*;
use migrations::*;

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
    pub db: PgPool,
    pub repositories: repository::RepositoryFactory,
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
    // Validate batch size
    if items.len() > 1000 {
        return Err(AppError::ValidationError("Batch size too large (max 1000 segments)".to_string()));
    }

    let mut added_count = 0;
    for item in items.into_inner() {
        let db_segment: DbDrawSegment = item.into();

        sqlx::query(
            "INSERT INTO draw_segments (id, board_id, user_id, points, color, width, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&db_segment.id)
        .bind(&db_segment.board_id)
        .bind(&db_segment.user_id)
        .bind(&db_segment.points)
        .bind(&db_segment.color)
        .bind(&db_segment.width)
        .bind(&db_segment.created_at)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to save drawing batch: {}", e)))?;

        added_count += 1;
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "added_count": added_count
    })))
}

#[post("/draw/batch")]
async fn add_draw_batch(state: Data<AppState>, items: web::Json<Vec<DrawSegment>>) -> impl Responder {
    // Validate batch size
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
async fn get_boards(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let repo = state.repositories.board_repository();
    let boards = repo.find_all().await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    let boards: Vec<Board> = boards.into_iter().map(|b| b.into()).collect();
    Ok(HttpResponse::Ok().json(boards))
}

#[post("/boards")]
async fn create_board(state: Data<AppState>, req: web::Json<CreateBoardRequest>) -> Result<impl Responder, AppError> {
    // Validate input
    req.validate().map_err(AppError::ValidationError)?;

    // Create board using repository
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

#[get("/migrations/status")]
async fn get_migration_status_endpoint(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let status = get_migration_status(&state.db)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to get migration status: {}", e)))?;

    Ok(HttpResponse::Ok().json(status))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    println!("Starting server at http://{}", addr);

    // Set up database connection
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/whiteboard".to_string());

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    // Run database migrations
    println!("Running database migrations...");
    run_migrations(&db).await.expect("Failed to run migrations");
    println!("Migrations completed successfully");

    let app_state = Data::new(AppState {
        db: db.clone(),
        repositories: repository::RepositoryFactory::new(db),
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
            .service(get_migration_status_endpoint)
            .route("/ws", web::get().to(ws_handler))
    })
    .bind(addr)?
    .run()
    .await
}
