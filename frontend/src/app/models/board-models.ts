export interface Board {
  id: string;
  name: string;
  owner_id: string;
  created_at: number;
  updated_at: number;
  is_public: boolean;
}

export interface CreateBoardRequest {
  name: string;
  is_public: boolean;
}

export interface BoardPage {
  items: Board[];
  next_cursor?: string;
  limit: number;
}
