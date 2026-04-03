export interface DrawPoint {
  x: number;
  y: number;
}

export interface DrawSegment {
  id: string;
  userId: string;
  points: DrawPoint[];
  color: string;
  width: number;
}
