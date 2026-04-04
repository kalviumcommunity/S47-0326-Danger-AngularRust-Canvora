/**
 * Base URL for the Rust API (no trailing slash).
 * Operators: align with `BACKEND_PORT` / `PUBLIC_API_BASE_URL` from `.env.example` when not using defaults.
 * Docker: pass `API_BASE_URL` build-arg (`Dockerfile.frontend`) and adjust this constant or add a runtime config step if needed.
 */
export const API_BASE_URL = 'http://127.0.0.1:8080';

export function isApiRequestUrl(url: string): boolean {
  return url.startsWith(API_BASE_URL);
}

/** Derive WebSocket origin from the HTTP API base (e.g. `http://localhost:8080` → `ws://localhost:8080`). */
export function apiBaseToWsBase(httpBase: string): string {
  if (httpBase.startsWith('https://')) {
    return `wss://${httpBase.slice('https://'.length)}`;
  }
  if (httpBase.startsWith('http://')) {
    return `ws://${httpBase.slice('http://'.length)}`;
  }
  return httpBase;
}
