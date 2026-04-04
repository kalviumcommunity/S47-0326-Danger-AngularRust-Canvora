use serde::{Deserialize, Serialize};
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
pub struct Board {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub is_public: bool,
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