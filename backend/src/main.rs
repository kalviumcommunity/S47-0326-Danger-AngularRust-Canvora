mod models;
mod migrations;
mod repository;
mod jwt;
mod ws;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse, post, get};
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::web::Data;
use actix_web::HttpRequest;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc, Duration};
use dotenv::dotenv;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use models::*;
use migrations::*;
use repository::*;
use jwt::*;
use ws::{ws_handler, WsHub};

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

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub repositories: RepositoryFactory,
    pub start_time: SystemTime,
    pub ws_hub: WsHub,
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

fn generate_jwt(user_id: &str, email: &str) -> Result<String, JwtError> {
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        email: email.to_owned(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
}

fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(jwt_secret.as_ref()), &validation)?;
    Ok(token_data.claims)
}

async fn extract_user_id_from_request(req: &HttpRequest, state: &Data<AppState>) -> Result<String, AppError> {
    let auth_header = req.headers().get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::ValidationError("Missing or invalid Authorization header".to_string()))?;

    let claims = validate_jwt(auth_header)
        .map_err(|_| AppError::ValidationError("Invalid token".to_string()))?;

    Ok(claims.sub)
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

#[post("/login")]
async fn login(state: Data<AppState>, req: web::Json<LoginRequest>) -> Result<impl Responder, AppError> {
    let user_repo = state.repositories.user_repository();

    // Find user by email
    let db_user = user_repo.find_by_email(&req.email).await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::ValidationError("Invalid credentials".to_string()))?;

    // Verify password
    let password_valid = verify_password(&req.password, &db_user.password_hash)
        .map_err(|e| AppError::InternalError(format!("Password verification error: {}", e)))?;

    if !password_valid {
        return Err(AppError::ValidationError("Invalid credentials".to_string()));
    }

    // Create JWT token
    let token = create_jwt(&db_user)
        .map_err(|e| AppError::InternalError(format!("Token creation error: {}", e)))?;

    let expires_at = (Utc::now() + Duration::hours(24)).timestamp() as u64;
    let user = User::from(db_user);

    let response = AuthResponse {
        user,
        token,
        expires_at,
    };

    Ok(HttpResponse::Ok().json(response))
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

#[get("/migrations/status")]
async fn get_migration_status_endpoint(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let status = get_migration_status(&state.db)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to get migration status: {}", e)))?;

    Ok(HttpResponse::Ok().json(status))
}

#[post("/auth/register")]
async fn register(state: Data<AppState>, req: web::Json<RegisterRequest>) -> Result<impl Responder, AppError> {
    // Validate input
    if req.email.trim().is_empty() || req.name.trim().is_empty() || req.password.len() < 6 {
        return Err(AppError::ValidationError("Invalid registration data".to_string()));
    }

    let repo = state.repositories.user_repository();

    // Check if user already exists
    if repo.find_by_email(&req.email).await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?
        .is_some() {
        return Err(AppError::ValidationError("User already exists".to_string()));
    }

    // Hash password
    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|_| AppError::InternalError("Failed to hash password".to_string()))?;

    // Create user
    let db_user = repo.create_user(&req.email, &req.name, &password_hash).await
        .map_err(|e| AppError::InternalError(format!("Failed to create user: {}", e)))?;

    let user = User::from(db_user);
    let token = generate_jwt(&user.id, &user.email)
        .map_err(|_| AppError::InternalError("Failed to generate token".to_string()))?;

    let expires_at = (Utc::now() + chrono::Duration::hours(24)).timestamp() as u64;

    Ok(HttpResponse::Created().json(AuthResponse {
        user,
        token,
        expires_at,
    }))
}

#[post("/auth/login")]
async fn login(state: Data<AppState>, req: web::Json<LoginRequest>) -> Result<impl Responder, AppError> {
    let repo = state.repositories.user_repository();

    // Find user by email
    let db_user = repo.find_by_email(&req.email).await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::ValidationError("Invalid credentials".to_string()))?;

    // Verify password
    if !verify(&req.password, &db_user.password_hash)
        .map_err(|_| AppError::InternalError("Failed to verify password".to_string()))? {
        return Err(AppError::ValidationError("Invalid credentials".to_string()));
    }

    let user = User::from(db_user);
    let token = generate_jwt(&user.id, &user.email)
        .map_err(|_| AppError::InternalError("Failed to generate token".to_string()))?;

    let expires_at = (Utc::now() + chrono::Duration::hours(24)).timestamp() as u64;

    Ok(HttpResponse::Ok().json(AuthResponse {
        user,
        token,
        expires_at,
    }))
}

#[get("/auth/me")]
async fn get_me(req: HttpRequest, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let user_id = extract_user_id_from_request(&req, &state).await?;
    let uuid = sqlx::types::Uuid::parse_str(&user_id)
        .map_err(|_| AppError::ValidationError("Invalid user ID".to_string()))?;

    let repo = state.repositories.user_repository();
    let db_user = repo.find_by_id(uuid).await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(User::from(db_user)))
}

fn require_env(key: &str) -> String {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            eprintln!(
                "FATAL: environment variable `{key}` is missing or empty.\n\
                 Copy the repository `.env.example` to `.env` and set required values."
            );
            std::process::exit(1);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    println!("Starting server at http://{}", addr);

    let database_url = require_env("DATABASE_URL");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to connect to database: {}", e)))?;

    run_migrations(&db)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to run migrations: {}", e)))?;
    println!("Migrations completed successfully");

    let app_state = Data::new(AppState {
        db: db.clone(),
        repositories: RepositoryFactory::new(db),
        start_time: SystemTime::now(),
        ws_hub: WsHub::new(),
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
            .service(register)
            .service(login)
            .service(get_me)
            .service(add_draw)
            .service(add_draw_batch)
            .service(get_boards)
            .service(create_board)
            .service(get_board)
            .service(get_board_drawings)
            .service(get_migration_status_endpoint)
            .route("/ws/{room}", web::get().to(ws_handler))
    })
    .bind(addr)?
    .run()
    .await
}
