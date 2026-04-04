import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { DrawSegment, DrawPoint } from './models/draw-models';
import { ApiService } from './api.service';

export interface PenSettings {
  color: string;
  width: number;
}

@Injectable({
  providedIn: 'root'
})
export class WhiteboardStateService {
  private segmentsSubject = new BehaviorSubject<DrawSegment[]>([]);
  public segments$ = this.segmentsSubject.asObservable();

  private penSettingsSubject = new BehaviorSubject<PenSettings>({
    color: '#000000',
    width: 2
  });
  public penSettings$ = this.penSettingsSubject.asObservable();

  constructor(private apiService: ApiService) {
    // Load from localStorage or initialize
    this.loadState();
  }

  addSegment(segment: DrawSegment): void {
    const current = this.segmentsSubject.value;
    this.segmentsSubject.next([...current, segment]);
    this.saveState();
  }

  setSegments(segments: DrawSegment[]): void {
    this.segmentsSubject.next(segments);
    this.saveState();
  }

  updatePenSettings(settings: Partial<PenSettings>): void {
    const current = this.penSettingsSubject.value;
    const updated = { ...current, ...settings };
    this.penSettingsSubject.next(updated);
    this.saveState();
  }

  clearSegments(): void {
    this.segmentsSubject.next([]);
    this.saveState();
  }

  loadDrawingsForBoard(boardId: string): Observable<DrawSegment[]> {
    return this.apiService.getBoardDrawings(boardId);
  }

  get segments(): DrawSegment[] {
    return this.segmentsSubject.value;
  }

  get penSettings(): PenSettings {
    return this.penSettingsSubject.value;
  }

  private saveState(): void {
    localStorage.setItem('whiteboard-segments', JSON.stringify(this.segments));
    localStorage.setItem('whiteboard-pen-settings', JSON.stringify(this.penSettings));
  }

  private loadState(): void {
    const segmentsData = localStorage.getItem('whiteboard-segments');
    if (segmentsData) {
      try {
        this.segmentsSubject.next(JSON.parse(segmentsData));
      } catch (e) {
        console.error('Failed to load segments', e);
      }
    }

    const penData = localStorage.getItem('whiteboard-pen-settings');
    if (penData) {
      try {
        this.penSettingsSubject.next(JSON.parse(penData));
      } catch (e) {
        console.error('Failed to load pen settings', e);
      }
    }
  }
}