import { Injectable, OnDestroy } from '@angular/core';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';
import { BehaviorSubject, Observable, Subject, timer } from 'rxjs';
import { filter, map, mergeMap, retryWhen, share, takeUntil } from 'rxjs/operators';
import { API_BASE_URL, apiBaseToWsBase } from './api-config';

export type RealtimeTopic = 'cursor' | 'draw' | 'chat';

export interface BoardRealtimeEnvelope {
  type: RealtimeTopic;
  payload: unknown;
}

@Injectable({ providedIn: 'root' })
export class BoardRealtimeService implements OnDestroy {
  private readonly destroy$ = new Subject<void>();
  private socket?: WebSocketSubject<BoardRealtimeEnvelope>;
  private readonly connectionSubject = new BehaviorSubject<'connected' | 'disconnected'>('disconnected');
  readonly connectionState$ = this.connectionSubject.asObservable();
  private readonly outbox: BoardRealtimeEnvelope[] = [];
  private readonly maxOutbox = 200;

  private wsUrl(room: string): string {
    return `${apiBaseToWsBase(API_BASE_URL)}/ws/${encodeURIComponent(room)}`;
  }

  /**
   * Live stream for a board room with exponential backoff reconnect (1s → 2s → … cap 30s).
   */
  connect(room: string): Observable<BoardRealtimeEnvelope> {
    this.socket = webSocket<BoardRealtimeEnvelope>({
      url: this.wsUrl(room),
      openObserver: {
        next: () => {
          this.connectionSubject.next('connected');
          this.flushOutbox();
        }
      },
      closeObserver: {
        next: () => this.connectionSubject.next('disconnected')
      },
      deserializer: (e: MessageEvent) => {
        const raw = e.data;
        let text: string;
        if (typeof raw === 'string') {
          text = raw;
        } else if (raw instanceof ArrayBuffer) {
          text = new TextDecoder().decode(raw);
        } else if (ArrayBuffer.isView(raw)) {
          text = new TextDecoder().decode(raw);
        } else {
          text = String(raw);
        }
        return JSON.parse(text) as BoardRealtimeEnvelope;
      },
      serializer: (v: BoardRealtimeEnvelope) => JSON.stringify(v)
    });

    return this.socket.pipe(
      retryWhen(errors =>
        errors.pipe(
          mergeMap((_, attempt) => {
            this.connectionSubject.next('disconnected');
            const delayMs = Math.min(30_000, 1000 * Math.pow(2, attempt));
            return timer(delayMs);
          })
        )
      ),
      takeUntil(this.destroy$),
      share()
    );
  }

  cursorUpdates$(stream: Observable<BoardRealtimeEnvelope>): Observable<unknown> {
    return stream.pipe(
      filter(m => m.type === 'cursor'),
      map(m => m.payload)
    );
  }

  elementCreations$(stream: Observable<BoardRealtimeEnvelope>): Observable<unknown> {
    return stream.pipe(
      filter(m => m.type === 'draw'),
      map(m => m.payload)
    );
  }

  chatMessages$(stream: Observable<BoardRealtimeEnvelope>): Observable<unknown> {
    return stream.pipe(
      filter(m => m.type === 'chat'),
      map(m => m.payload)
    );
  }

  send(message: BoardRealtimeEnvelope): void {
    if (typeof navigator !== 'undefined' && navigator.onLine === false) {
      if (this.outbox.length < this.maxOutbox) {
        this.outbox.push(message);
      }
      return;
    }
    this.socket?.next(message);
  }

  private flushOutbox(): void {
    while (this.outbox.length > 0 && this.socket) {
      const next = this.outbox.shift();
      if (next) {
        this.socket.next(next);
      }
    }
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
    this.socket?.complete();
  }
}
