import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Board, BoardPage, CreateBoardRequest } from './models/board-models';
import { DrawSegment } from './models/draw-models';
import { ErrorService } from './error.service';

const API_BASE = 'http://127.0.0.1:8080';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  constructor(private http: HttpClient, private errorService: ErrorService) {}

  getBoards(): Observable<BoardPage> {
    return this.http.get<BoardPage>(`${API_BASE}/boards?limit=25`);
  }

  createBoard(request: CreateBoardRequest): Observable<Board> {
    return this.http.post<Board>(`${API_BASE}/boards`, request);
  }

  getBoardDrawings(boardId: string): Observable<DrawSegment[]> {
    return this.http.get<DrawSegment[]>(`${API_BASE}/boards/${boardId}/drawings`);
  }

  saveDrawingSegment(segment: DrawSegment): Observable<unknown> {
    return this.http.post(`${API_BASE}/draw`, segment);
  }

  saveDrawingBatch(segments: DrawSegment[]): Observable<unknown> {
    return this.http.post(`${API_BASE}/draw/batch`, segments);
  }
}
