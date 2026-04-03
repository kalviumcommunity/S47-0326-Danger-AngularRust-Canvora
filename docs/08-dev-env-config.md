# Issue #8: Angular + Rust Development Environment Config

## Goal
Document and configure a minimal, functional development environment for this full-stack app.

## Frontend
- Node 18+ and npm
- `cd frontend`
- `npm i`
- `npx ng serve --port 4300` (default)

### Optional helper scripts (repo root package.json)
```json
"scripts": {
  "frontend:install": "cd frontend && npm install",
  "frontend:start": "cd frontend && npx ng serve --port 4300",
  "backend:start": "cd backend && cargo run",
  "dev": "concurrently \"npm run frontend:start\" \"npm run backend:start\""
}
```

## Backend
- Rust 1.70+ 
- `cd backend`
- `cargo run`

## Notes
- Use `--port` for Angular if 4200 is unavailable.
- Keep web socket and API URLs aligned (backend default: `http://127.0.0.1:8080`).
