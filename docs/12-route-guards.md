# Issue #12: Implement Route Protection & Guards in Angular

## Goal
Add simple route guard and basic auth routing to keep core app flow organized.

## Changes
- `auth.guard.ts`: `authGuard` with basic in-memory boolean
- `login.ts`: placeholder login component
- `app.routes.ts`: add routes for `/login`, `/whiteboard`, and fallback
- `app.ts`: use `RouterOutlet` standalone route rendering
- `app.html`: navigation links + router outlet

## Result
- `/whiteboard` is protected by `authGuard`.
- `authGuard` currently always allows (`true`) but is an extension point.
- routes are functional using Angular standalone router.
