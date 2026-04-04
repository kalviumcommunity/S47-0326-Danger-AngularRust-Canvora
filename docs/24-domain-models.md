# 24. Backend: Define Core Domain Models & Serde Types

## Overview
Established comprehensive domain models for the whiteboard application with proper Serde serialization support for API communication.

## Changes Made

### New Models File (`backend/src/models.rs`)
- Created dedicated `models.rs` module for domain entities
- Defined core business objects: `User`, `Board`, `Session`, `DrawSegment`, `DrawPoint`
- Added API request/response types: `ApiResponse`, `CreateBoardRequest`, `LoginRequest`, `LoginResponse`

### Domain Models

#### User
- `id`, `email`, `name` fields
- Timestamps for creation and updates
- Constructor method for new users

#### Board
- `id`, `name`, `owner_id` fields
- Public/private visibility flag
- Timestamps for creation and updates
- Constructor method for new boards

#### Session
- Authentication session management
- Token-based with expiration
- Links user to board access

#### DrawSegment (Enhanced)
- Added `board_id` and `created_at` fields
- Constructor method for new segments
- Maintains existing drawing data structure

### API Types
- `ApiResponse<T>`: Generic response wrapper for API consistency
- Request/Response types for board creation and authentication
- Proper error handling structures

### Main.rs Updates
- Modularized code by moving models to separate file
- Updated imports to use `models::*`
- Maintained backward compatibility with existing endpoints

## Technical Details
- All models implement `Debug`, `Serialize`, `Deserialize`, `Clone` traits
- UUID generation for unique identifiers
- Unix timestamp handling for time fields
- Constructor methods for easy object creation

## Benefits
- **Type Safety**: Strong typing for all domain entities
- **API Consistency**: Standardized request/response formats
- **Maintainability**: Separated concerns with dedicated models module
- **Extensibility**: Easy to add new domain objects and relationships

## Next Steps
- Issue #25: Construct REST Routing & Controller Layer