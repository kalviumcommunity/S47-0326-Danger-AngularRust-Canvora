import { Injectable } from '@angular/core';
import { BoardRepository } from './board.repository';
import { DrawingRepository } from './drawing.repository';

@Injectable({
  providedIn: 'root'
})
export class RepositoryFactory {
  constructor(
    private boardRepository: BoardRepository,
    private drawingRepository: DrawingRepository
  ) {}

  boards(): BoardRepository {
    return this.boardRepository;
  }

  drawings(): DrawingRepository {
    return this.drawingRepository;
  }
}