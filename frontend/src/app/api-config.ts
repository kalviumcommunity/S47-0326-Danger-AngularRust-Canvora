/** Base URL for the Rust API (no trailing slash). Kept in one place for interceptors and clients. */
export const API_BASE_URL = 'http://127.0.0.1:8080';

export function isApiRequestUrl(url: string): boolean {
  return url.startsWith(API_BASE_URL);
}
