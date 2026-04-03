# Issue #14: Optimize Angular CLI & Build Workflow

## Goal
Add sensible build and development npm scripts for easier contributor onboarding.

## Changes added
- `package.json` root scripts:
  - `frontend:build` (npm run from root)
  - `frontend:start`, `backend:start`, `dev`, `frontend:install`

## Run
- `npm run frontend:build`
- `npm run dev`

## Why
Simple local workflow with centralized commands should reduce CLI complexity.
