export interface DrawPoint {
  x: number;
  y: number;
}

export interface DrawSegment {
  id: string;
  board_id: string;
  user_id: string;
  points: DrawPoint[];
  color: string;
  width: number;
  created_at: number;
}
