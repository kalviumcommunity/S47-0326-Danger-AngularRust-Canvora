import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';

export interface AppError {
  message: string;
  timestamp: number;
  type: 'error' | 'warning' | 'info';
}

@Injectable({
  providedIn: 'root'
})
export class ErrorService {
  private errorSubject = new BehaviorSubject<AppError | null>(null);
  public error$ = this.errorSubject.asObservable();

  showError(message: string, type: 'error' | 'warning' | 'info' = 'error'): void {
    const error: AppError = {
      message,
      timestamp: Date.now(),
      type
    };
    this.errorSubject.next(error);

    // Auto-clear error after 5 seconds for non-error types
    if (type !== 'error') {
      setTimeout(() => {
        if (this.errorSubject.value === error) {
          this.clearError();
        }
      }, 5000);
    }
  }

  clearError(): void {
    this.errorSubject.next(null);
  }

  get currentError(): AppError | null {
    return this.errorSubject.value;
  }
}