use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<DbUser> for User {
    fn from(db: DbUser) -> Self {
        Self {
            id: db.id.to_string(),
            email: db.email,
            name: db.name,
            created_at: db.created_at.timestamp() as u64,
            updated_at: db.updated_at.timestamp() as u64,
        }
    }
}

impl From<User> for DbUser {
    fn from(user: User) -> Self {
        let created_at = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(user.created_at as i64, 0).unwrap_or_else(|| Utc::now().naive_utc()),
            Utc,
        );
        let updated_at = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(user.updated_at as i64, 0).unwrap_or_else(|| Utc::now().naive_utc()),
            Utc,
        );

        Self {
            id: Uuid::parse_str(&user.id).unwrap_or_else(|_| Uuid::new_v4()),
            email: user.email,
            name: user.name,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbBoard {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_public: bool,
}

impl From<DbBoard> for Board {
    fn from(db: DbBoard) -> Self {
        Self {
            id: db.id.to_string(),
            name: db.name,
            owner_id: db.owner_id.to_string(),
            created_at: db.created_at.timestamp() as u64,
            updated_at: db.updated_at.timestamp() as u64,
            is_public: db.is_public,
        }
    }
}

impl From<Board> for DbBoard {
    fn from(board: Board) -> Self {
        let created_at = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(board.created_at as i64, 0).unwrap_or_else(|| Utc::now().naive_utc()),
            Utc,
        );
        let updated_at = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(board.updated_at as i64, 0).unwrap_or_else(|| Utc::now().naive_utc()),
            Utc,
        );

        Self {
            id: Uuid::parse_str(&board.id).unwrap_or_else(|_| Uuid::new_v4()),
            name: board.name,
            owner_id: Uuid::parse_str(&board.owner_id).unwrap_or_else(|_| Uuid::new_v4()),
            created_at,
            updated_at,
            is_public: board.is_public,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub board_id: String,
    pub token: String,
    pub expires_at: u64,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DrawPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DrawSegment {
    pub id: String,
    pub board_id: String,
    pub user_id: String,
    pub points: Vec<DrawPoint>,
    pub color: String,
    pub width: f32,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbDrawSegment {
    pub id: Uuid,
    pub board_id: Uuid,
    pub user_id: Uuid,
    pub points: Vec<DrawPoint>,
    pub color: String,
    pub width: f32,
    pub created_at: DateTime<Utc>,
}

impl From<DbDrawSegment> for DrawSegment {
    fn from(db: DbDrawSegment) -> Self {
        Self {
            id: db.id.to_string(),
            board_id: db.board_id.to_string(),
            user_id: db.user_id.to_string(),
            points: db.points,
            color: db.color,
            width: db.width,
            created_at: db.created_at.timestamp() as u64,
        }
    }
}

impl From<DrawSegment> for DbDrawSegment {
    fn from(segment: DrawSegment) -> Self {
        let created_at = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(segment.created_at as i64, 0).unwrap_or_else(|| Utc::now().naive_utc()),
            Utc,
        );

        Self {
            id: Uuid::parse_str(&segment.id).unwrap_or_else(|_| Uuid::new_v4()),
            board_id: Uuid::parse_str(&segment.board_id).unwrap_or_else(|_| Uuid::new_v4()),
            user_id: Uuid::parse_str(&segment.user_id).unwrap_or_else(|_| Uuid::new_v4()),
            points: segment.points,
            color: segment.color,
            width: segment.width,
            created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateBoardRequest {
    pub name: String,
    pub is_public: bool,
}

impl CreateBoardRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Board name cannot be empty".to_string());
        }
        if self.name.len() > 100 {
            return Err("Board name too long (max 100 characters)".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
    pub expires_at: u64,
}

impl User {
    pub fn new(email: String, name: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email,
            name,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Board {
    pub fn new(name: String, owner_id: String, is_public: bool) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            owner_id,
            created_at: now,
            updated_at: now,
            is_public,
        }
    }
}

impl Session {
    pub fn new(user_id: String, board_id: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let expires_at = now + 86400; // 24 hours

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            board_id,
            token: uuid::Uuid::new_v4().to_string(),
            expires_at,
            created_at: now,
        }
    }
}

impl DrawSegment {
    pub fn new(board_id: String, user_id: String, points: Vec<DrawPoint>, color: String, width: f32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            board_id,
            user_id,
            points,
            color,
            width,
            created_at: now,
        }
    }
}

// Database Models for PostgreSQL
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DbUser {
    pub id: sqlx::types::Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DbBoard {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub owner_id: sqlx::types::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_public: bool,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DbSession {
    pub id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    pub board_id: sqlx::types::Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DbDrawSegment {
    pub id: sqlx::types::Uuid,
    pub board_id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    pub points: serde_json::Value, // Store as JSON in DB
    pub color: String,
    pub width: f32,
    pub created_at: DateTime<Utc>,
}

// Conversion implementations
impl From<DbUser> for User {
    fn from(db: DbUser) -> Self {
        User {
            id: db.id.to_string(),
            email: db.email,
            name: db.name,
            created_at: db.created_at.timestamp() as u64,
            updated_at: db.updated_at.timestamp() as u64,
        }
    }
}

impl From<User> for DbUser {
    fn from(user: User) -> Self {
        DbUser {
            id: sqlx::types::Uuid::parse_str(&user.id).unwrap_or_else(|_| sqlx::types::Uuid::new_v4()),
            email: user.email,
            name: user.name,
            created_at: DateTime::<Utc>::from_timestamp(user.created_at as i64, 0).unwrap_or_else(|| Utc::now()),
            updated_at: DateTime::<Utc>::from_timestamp(user.updated_at as i64, 0).unwrap_or_else(|| Utc::now()),
        }
    }
}

impl From<DbBoard> for Board {
    fn from(db: DbBoard) -> Self {
        Board {
            id: db.id.to_string(),
            name: db.name,
            owner_id: db.owner_id.to_string(),
            created_at: db.created_at.timestamp() as u64,
            updated_at: db.updated_at.timestamp() as u64,
            is_public: db.is_public,
        }
    }
}

impl From<Board> for DbBoard {
    fn from(board: Board) -> Self {
        DbBoard {
            id: sqlx::types::Uuid::parse_str(&board.id).unwrap_or_else(|_| sqlx::types::Uuid::new_v4()),
            name: board.name,
            owner_id: sqlx::types::Uuid::parse_str(&board.owner_id).unwrap_or_else(|_| sqlx::types::Uuid::new_v4()),
            created_at: DateTime::<Utc>::from_timestamp(board.created_at as i64, 0).unwrap_or_else(|| Utc::now()),
            updated_at: DateTime::<Utc>::from_timestamp(board.updated_at as i64, 0).unwrap_or_else(|| Utc::now()),
            is_public: board.is_public,
        }
    }
}

impl From<DbDrawSegment> for DrawSegment {
    fn from(db: DbDrawSegment) -> Self {
        let points: Vec<DrawPoint> = serde_json::from_value(db.points).unwrap_or_default();

        DrawSegment {
            id: db.id.to_string(),
            board_id: db.board_id.to_string(),
            user_id: db.user_id.to_string(),
            points,
            color: db.color,
            width: db.width,
            created_at: db.created_at.timestamp() as u64,
        }
    }
}

impl From<DrawSegment> for DbDrawSegment {
    fn from(segment: DrawSegment) -> Self {
        let points = serde_json::to_value(&segment.points).unwrap_or(serde_json::Value::Array(vec![]));

        DbDrawSegment {
            id: sqlx::types::Uuid::parse_str(&segment.id).unwrap_or_else(|_| sqlx::types::Uuid::new_v4()),
            board_id: sqlx::types::Uuid::parse_str(&segment.board_id).unwrap_or_else(|_| sqlx::types::Uuid::new_v4()),
            user_id: sqlx::types::Uuid::parse_str(&segment.user_id).unwrap_or_else(|_| sqlx::types::Uuid::new_v4()),
            points,
            color: segment.color,
            width: segment.width,
            created_at: DateTime::<Utc>::from_timestamp(segment.created_at as i64, 0).unwrap_or_else(|| Utc::now()),
        }
    }
}