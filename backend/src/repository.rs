use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Result};
use crate::models::*;

/// Generic repository trait for CRUD operations
#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>>;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn save(&self, entity: T) -> Result<T>;
    async fn update(&self, entity: T) -> Result<T>;
    async fn delete(&self, id: ID) -> Result<bool>;
    async fn exists(&self, id: ID) -> Result<bool>;
}

/// User repository implementation
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<DbUser, sqlx::types::Uuid> for UserRepository {
    async fn find_by_id(&self, id: sqlx::types::Uuid) -> Result<Option<DbUser>> {
        sqlx::query_as!(
            DbUser,
            "SELECT id, email, name, password_hash, created_at, updated_at FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<DbUser>> {
        sqlx::query_as!(
            DbUser,
            "SELECT id, email, name, password_hash, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn save(&self, user: DbUser) -> Result<DbUser> {
        sqlx::query_as!(
            DbUser,
            r#"
            INSERT INTO users (id, email, name, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, name, password_hash, created_at, updated_at
            "#,
            user.id,
            user.email,
            user.name,
            user.password_hash,
            user.created_at,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, user: DbUser) -> Result<DbUser> {
        sqlx::query_as!(
            DbUser,
            r#"
            UPDATE users
            SET email = $2, name = $3, password_hash = $4, updated_at = $5
            WHERE id = $1
            RETURNING id, email, name, password_hash, created_at, updated_at
            "#,
            user.id,
            user.email,
            user.name,
            user.password_hash,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: sqlx::types::Uuid) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn exists(&self, id: sqlx::types::Uuid) -> Result<bool> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count.unwrap_or(0) > 0)
    }
}

impl UserRepository {
    pub async fn find_by_email(&self, email: &str) -> Result<Option<DbUser>> {
        sqlx::query_as!(
            DbUser,
            "SELECT id, email, name, password_hash, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_user(&self, email: &str, name: &str, password_hash: &str) -> Result<DbUser> {
        let now = chrono::Utc::now();
        let user_id = sqlx::types::Uuid::new_v4();

        sqlx::query_as!(
            DbUser,
            r#"
            INSERT INTO users (id, email, name, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, name, password_hash, created_at, updated_at
            "#,
            user_id,
            email,
            name,
            password_hash,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await
    }
}

/// Board repository implementation
pub struct BoardRepository {
    pool: PgPool,
}

impl BoardRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<DbBoard, sqlx::types::Uuid> for BoardRepository {
    async fn find_by_id(&self, id: sqlx::types::Uuid) -> Result<Option<DbBoard>> {
        sqlx::query_as!(
            DbBoard,
            "SELECT id, name, owner_id, created_at, updated_at, is_public FROM boards WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<DbBoard>> {
        sqlx::query_as!(
            DbBoard,
            "SELECT id, name, owner_id, created_at, updated_at, is_public FROM boards ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn save(&self, board: DbBoard) -> Result<DbBoard> {
        sqlx::query_as!(
            DbBoard,
            r#"
            INSERT INTO boards (id, name, owner_id, created_at, updated_at, is_public)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, owner_id, created_at, updated_at, is_public
            "#,
            board.id,
            board.name,
            board.owner_id,
            board.created_at,
            board.updated_at,
            board.is_public
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, board: DbBoard) -> Result<DbBoard> {
        sqlx::query_as!(
            DbBoard,
            r#"
            UPDATE boards
            SET name = $2, owner_id = $3, updated_at = $4, is_public = $5
            WHERE id = $1
            RETURNING id, name, owner_id, created_at, updated_at, is_public
            "#,
            board.id,
            board.name,
            board.owner_id,
            board.updated_at,
            board.is_public
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: sqlx::types::Uuid) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM boards WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn exists(&self, id: sqlx::types::Uuid) -> Result<bool> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM boards WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count.unwrap_or(0) > 0)
    }
}

impl BoardRepository {
    pub async fn find_by_owner(&self, owner_id: sqlx::types::Uuid) -> Result<Vec<DbBoard>> {
        sqlx::query_as!(
            DbBoard,
            "SELECT id, name, owner_id, created_at, updated_at, is_public FROM boards WHERE owner_id = $1 ORDER BY created_at DESC",
            owner_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_public(&self) -> Result<Vec<DbBoard>> {
        sqlx::query_as!(
            DbBoard,
            "SELECT id, name, owner_id, created_at, updated_at, is_public FROM boards WHERE is_public = true ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_page(&self, cursor_created_at: Option<DateTime<Utc>>, limit: i64) -> Result<Vec<DbBoard>> {
        sqlx::query_as!(
            DbBoard,
            "SELECT id, name, owner_id, created_at, updated_at, is_public FROM boards
             WHERE $1::timestamptz IS NULL OR created_at < $1
             ORDER BY created_at DESC
             LIMIT $2",
            cursor_created_at,
            limit
        )
        .fetch_all(&self.pool)
        .await
    }
}

/// Draw segment repository implementation
pub struct DrawSegmentRepository {
    pool: PgPool,
}

impl DrawSegmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<DbDrawSegment, sqlx::types::Uuid> for DrawSegmentRepository {
    async fn find_by_id(&self, id: sqlx::types::Uuid) -> Result<Option<DbDrawSegment>> {
        sqlx::query_as!(
            DbDrawSegment,
            "SELECT id, board_id, user_id, points, color, width, created_at FROM draw_segments WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<DbDrawSegment>> {
        sqlx::query_as!(
            DbDrawSegment,
            "SELECT id, board_id, user_id, points, color, width, created_at FROM draw_segments ORDER BY created_at ASC"
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn save(&self, segment: DbDrawSegment) -> Result<DbDrawSegment> {
        sqlx::query_as!(
            DbDrawSegment,
            r#"
            INSERT INTO draw_segments (id, board_id, user_id, points, color, width, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, board_id, user_id, points, color, width, created_at
            "#,
            segment.id,
            segment.board_id,
            segment.user_id,
            segment.points,
            segment.color,
            segment.width,
            segment.created_at
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, segment: DbDrawSegment) -> Result<DbDrawSegment> {
        // Draw segments are typically immutable once created
        // This is a placeholder implementation
        self.save(segment).await
    }

    async fn delete(&self, id: sqlx::types::Uuid) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM draw_segments WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn exists(&self, id: sqlx::types::Uuid) -> Result<bool> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM draw_segments WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(result.count.unwrap_or(0) > 0)
    }
}

impl DrawSegmentRepository {
    pub async fn find_by_board(&self, board_id: sqlx::types::Uuid) -> Result<Vec<DbDrawSegment>> {
        sqlx::query_as!(
            DbDrawSegment,
            "SELECT id, board_id, user_id, points, color, width, created_at FROM draw_segments WHERE board_id = $1 ORDER BY created_at ASC",
            board_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_user(&self, user_id: sqlx::types::Uuid) -> Result<Vec<DbDrawSegment>> {
        sqlx::query_as!(
            DbDrawSegment,
            "SELECT id, board_id, user_id, points, color, width, created_at FROM draw_segments WHERE user_id = $1 ORDER BY created_at ASC",
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn save_batch(&self, segments: Vec<DbDrawSegment>) -> Result<Vec<DbDrawSegment>> {
        let mut saved_segments = Vec::new();

        for segment in segments {
            let saved = self.save(segment).await?;
            saved_segments.push(saved);
        }

        Ok(saved_segments)
    }
}

/// Repository factory for creating repository instances
pub struct RepositoryFactory {
    pool: PgPool,
}

impl RepositoryFactory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn user_repository(&self) -> UserRepository {
        UserRepository::new(self.pool.clone())
    }

    pub fn board_repository(&self) -> BoardRepository {
        BoardRepository::new(self.pool.clone())
    }

    pub fn draw_segment_repository(&self) -> DrawSegmentRepository {
        DrawSegmentRepository::new(self.pool.clone())
    }
}