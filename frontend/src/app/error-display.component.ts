import { Component, OnInit, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Subscription } from 'rxjs';
import { ErrorService, AppError } from './error.service';

@Component({
  selector: 'app-error-display',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div *ngIf="currentError" class="error-banner" [ngClass]="currentError.type">
      <span class="error-message">{{ currentError.message }}</span>
      <button type="button" class="close-btn" (click)="dismissError()">&times;</button>
    </div>
  `,
  styles: [`
    .error-banner {
      position: fixed;
      top: 20px;
      right: 20px;
      padding: 12px 16px;
      border-radius: 4px;
      color: white;
      font-weight: 500;
      display: flex;
      align-items: center;
      gap: 12px;
      z-index: 10000;
      min-width: 300px;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    }

    .error {
      background-color: #e74c3c;
    }

    .warning {
      background-color: #f39c12;
    }

    .info {
      background-color: #3498db;
    }

    .error-message {
      flex: 1;
    }

    .close-btn {
      background: none;
      border: none;
      color: white;
      font-size: 20px;
      cursor: pointer;
      padding: 0;
      width: 20px;
      height: 20px;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .close-btn:hover {
      opacity: 0.8;
    }
  `]
})
export class ErrorDisplayComponent implements OnInit, OnDestroy {
  currentError: AppError | null = null;
  private subscription?: Subscription;

  constructor(private errorService: ErrorService) {}

  ngOnInit() {
    this.subscription = this.errorService.error$.subscribe((error: AppError | null) => {
      this.currentError = error;
    });
  }

  ngOnDestroy() {
    this.subscription?.unsubscribe();
  }

  dismissError() {
    this.errorService.clearError();
  }
}