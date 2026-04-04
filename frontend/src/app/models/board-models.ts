import type { Board, CreateBoardRequest, PaginatedBoardsResponse } from '../../core/api-types';

export type { Board, CreateBoardRequest };

/** Cursor page of boards (Rust: PaginatedBoardsResponse). */
export type BoardPage = PaginatedBoardsResponse;
