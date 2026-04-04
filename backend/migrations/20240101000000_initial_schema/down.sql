-- Rollback migration: Drop all tables in reverse order

DROP TABLE IF EXISTS draw_segments;
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS boards;
DROP TABLE IF EXISTS users;