# Issue #9: Structure Monorepo Project for Whiteboard App

## Objective
Set up consistent top-level monorepo structure and workflow scripts to manage frontend and backend easily.

## Repository layout
- `/frontend` - Angular project
- `/backend` - Rust project
- `/docs` - architecture and process docs
- `/scripts` - automation and utilities

## Root package commands
- `npm run frontend:install`
- `npm run frontend:start`
- `npm run backend:start`
- `npm run dev` (runs frontend + backend concurrently)

## Notes
- `frontend` and `backend` remain independent but can be started together.
- Keep minimal linking until core functionality works.
