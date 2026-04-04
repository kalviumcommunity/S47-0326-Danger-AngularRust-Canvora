import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiService } from '../api.service';
import { DrawSegment } from '../models/draw-models';

@Injectable({
  providedIn: 'root'
})
export class DrawingRepository {
  constructor(private api: ApiService) {}

  findByBoard(boardId: string): Observable<DrawSegment[]> {
    return this.api.getBoardDrawings(boardId);
  }

  save(segment: DrawSegment): Observable<unknown> {
    return this.api.saveDrawingSegment(segment);
  }

  saveBatch(segments: DrawSegment[]): Observable<unknown> {
    return this.api.saveDrawingBatch(segments);
  }
}