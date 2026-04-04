-- Migration: Add password_hash column to users table for authentication
-- Run this SQL to add password hashing support

ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255) NOT NULL DEFAULT '';