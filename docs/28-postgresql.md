# Issue #28: PostgreSQL Integration

## Overview
Migrated the whiteboard application from in-memory storage to PostgreSQL database persistence with full CRUD operations.

## Changes Made

### Backend (Rust/Actix-web + SQLx)

#### Database Models
- Added `DbUser`, `DbBoard`, `DbSession`, `DbDrawSegment` structs with SQLx `FromRow` trait
- Implemented conversion traits (`From`/`Into`) between API models and database models
- Used PostgreSQL `UUID` type for primary keys
- Stored drawing points as `JSONB` for efficient querying

#### Database Connection
- Replaced `Arc<Mutex<Vec<T>>>` in-memory storage with `PgPool` connection pool
- Added database URL configuration via `DATABASE_URL` environment variable
- Implemented connection pooling with max 5 connections

#### Updated Handlers
All API endpoints now use database operations instead of in-memory storage:

- `GET /boards` - Query all boards ordered by creation date
- `POST /boards` - Insert new board with validation
- `GET /boards/{id}` - Query specific board by UUID
- `GET /boards/{id}/drawings` - Query drawings for specific board
- `POST /draw` - Insert single drawing segment
- `POST /draw/batch` - Insert multiple drawing segments in transaction
- `GET /state` - Query all drawing segments

#### Database Schema
Created comprehensive PostgreSQL schema with:
- **users** table: User accounts with email/name
- **boards** table: Whiteboard boards with ownership and visibility
- **sessions** table: User sessions with tokens and expiration
- **draw_segments** table: Drawing data with JSON points storage

#### Indexes and Performance
- Primary key indexes on all tables
- Foreign key constraints with cascade deletes
- Performance indexes on frequently queried columns:
  - `boards.owner_id`, `boards.created_at`
  - `sessions.token`, `sessions.expires_at`
  - `draw_segments.board_id`, `draw_segments.created_at`

## Database Setup

### Prerequisites
- PostgreSQL server running
- Database created: `whiteboard`

### Environment Variables
```bash
DATABASE_URL=postgres://username:password@localhost/whiteboard
```

### Schema Migration
Run the SQL in `backend/migrations/001_initial_schema.sql` to create tables and indexes.

## Technical Details

### Connection Pooling
```rust
let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect to database");
```

### Data Conversion
```rust
// API model to DB model
let db_segment: DbDrawSegment = segment.into();

// DB model to API model
let segment: DrawSegment = db_segment.into();
```

### Query Examples
```rust
// Insert board
sqlx::query("INSERT INTO boards (id, name, owner_id, ...) VALUES ($1, $2, $3, ...)")
    .bind(&board.id)
    .execute(&state.db)
    .await?;

// Query with joins
sqlx::query_as::<_, DbBoard>("SELECT * FROM boards WHERE id = $1")
    .bind(&uuid)
    .fetch_optional(&state.db)
    .await?;
```

## Benefits
- **Persistence**: Data survives server restarts
- **Scalability**: Multiple server instances can share database
- **Concurrency**: Proper transaction handling for concurrent operations
- **Performance**: Indexed queries for fast data retrieval
- **Reliability**: ACID compliance for data integrity
- **Query Power**: JSON queries on drawing points for advanced features

## Migration Path
- Existing in-memory data is lost (expected for MVP)
- API contracts remain unchanged
- All endpoints maintain backward compatibility
- Error handling improved with database-specific errors

## Next Steps
Ready for #29: Database migrations and #30: CRUD repository pattern