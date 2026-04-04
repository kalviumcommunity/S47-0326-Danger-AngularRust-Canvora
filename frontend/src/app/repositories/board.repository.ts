import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiService } from '../api.service';
import { Board, BoardPage, CreateBoardRequest } from '../models/board-models';

@Injectable({
  providedIn: 'root'
})
export class BoardRepository {
  constructor(private api: ApiService) {}

  findAll(): Observable<BoardPage> {
    return this.api.getBoards();
  }

  create(board: CreateBoardRequest): Observable<Board> {
    return this.api.createBoard(board);
  }

  findById(id: string): Observable<Board> {
    // Note: This would need a backend endpoint
    throw new Error('Not implemented');
  }
}