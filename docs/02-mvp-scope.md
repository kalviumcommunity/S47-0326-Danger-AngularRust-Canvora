# Minimum Viable Product (MVP) Scope

## Core Features (MVP)
- Real-time multi-user whiteboard (drawing, shapes, sticky notes)
- Live cursors
- Basic user authentication (login/signup)
- Board creation, join, and basic access control (owner, collaborator)
- WebSocket-based sync (instant updates)
- Simple version history (undo/redo per session)

## Optional/Post-MVP Features
- Advanced permissions (viewer, editor, admin)
- Board templates, export/import
- Comments, chat, reactions
- Deep versioning (timeline, branching)
- Integrations (Google Drive, Slack)
- Analytics, audit logs

## User Roles & Flows
- Owner: creates board, manages access
- Collaborator: edits board
- Flows: sign up/login → create/join board → edit in real-time → see live cursors/changes → undo/redo

## Technical Requirements
- Angular SPA (canvas, WebSocket client, auth UI)
- Rust backend (REST for auth/boards, WebSocket for sync)
- PostgreSQL (users, boards, sessions)
- Basic access control (board-level)
- Stateless REST APIs + stateful WebSocket sessions
- Simple in-memory or DB-backed version history

---

This document defines the MVP scope for the whiteboard application.
