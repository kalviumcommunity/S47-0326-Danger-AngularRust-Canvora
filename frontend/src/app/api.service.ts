import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Board, BoardPage, CreateBoardRequest } from './models/board-models';
import { DrawSegment } from './models/draw-models';
import { ErrorService } from './error.service';
import { API_BASE_URL } from './api-config';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  constructor(private http: HttpClient, private errorService: ErrorService) {}

  getBoards(): Observable<BoardPage> {
    return this.http.get<BoardPage>(`${API_BASE_URL}/boards?limit=25`);
  }

  createBoard(request: CreateBoardRequest): Observable<Board> {
    return this.http.post<Board>(`${API_BASE_URL}/boards`, request);
  }

  getBoardDrawings(boardId: string): Observable<DrawSegment[]> {
    return this.http.get<DrawSegment[]>(`${API_BASE_URL}/boards/${boardId}/drawings`);
  }

  saveDrawingSegment(segment: DrawSegment): Observable<unknown> {
    return this.http.post(`${API_BASE_URL}/draw`, segment);
  }

  saveDrawingBatch(segments: DrawSegment[]): Observable<unknown> {
    return this.http.post(`${API_BASE_URL}/draw/batch`, segments);
  }
}
