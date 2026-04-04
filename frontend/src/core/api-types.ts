/** API wire types — keep in sync with Rust `#[derive(TS)]` in `backend/src/models.rs`. Regenerate with: `cd backend && cargo test export_typescript_bindings`. */

export type User = {
  id: string;
  email: string;
  name: string;
  created_at: number;
  updated_at: number;
};

export type LoginRequest = {
  email: string;
  password: string;
};

export type RegisterRequest = {
  email: string;
  name: string;
  password: string;
};

export type AuthResponse = {
  user: User;
  token: string;
  expires_at: number;
};

export type Board = {
  id: string;
  name: string;
  owner_id: string;
  created_at: number;
  updated_at: number;
  is_public: boolean;
};

export type CreateBoardRequest = {
  name: string;
  is_public: boolean;
};

export type DrawPoint = {
  x: number;
  y: number;
};

export type DrawSegment = {
  id: string;
  board_id: string;
  user_id: string;
  points: Array<DrawPoint>;
  color: string;
  width: number;
  created_at: number;
};

export type PaginatedBoardsResponse = {
  items: Array<Board>;
  next_cursor?: string;
  limit: number;
};
