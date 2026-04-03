# Issue #6: High-Level Design (HLD)

## Purpose
This document defines a simplified architecture for the real-time collaborative whiteboard app, focusing on minimal implementation and clear behavior.

## Components
- Frontend (Angular)
  - `AppComponent` as root
  - `WhiteboardComponent` for drawing and event capture
  - `WebsocketService` (future) for RX connection and broadcast hooks
- Backend (Rust)
  - HTTP service (setup in `main.rs`)
  - WebSocket endpoint for real-time updates
  - Shared state in-memory for simplicity

## Data model (simplified)
- `DrawCommand` (line, coords, userId, color, width)
- `BoardState` (array of commands)

## Workflow
1. User draws on canvas.
2. Frontend sends command to backend via WS.
3. Backend broadcasts command to all sockets.
4. Frontend applies received command to local canvas.

## Non-goals for MVP
- No persistence DB (in-memory only)
- No authentication
- No complex conflict resolution
