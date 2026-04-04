import { Injectable } from '@angular/core';
import { HttpInterceptor, HttpRequest, HttpHandler, HttpEvent, HttpErrorResponse } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { ErrorService } from './error.service';

@Injectable()
export class ErrorInterceptor implements HttpInterceptor {
  constructor(private errorService: ErrorService) {}

  intercept(request: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    return next.handle(request).pipe(
      catchError((error: HttpErrorResponse) => {
        if (error.status === 401) {
          return throwError(() => error);
        }

        let message = 'An HTTP error occurred';

        if (error.error?.error) {
          message = error.error.error;
        } else if (error.message) {
          message = error.message;
        } else if (error.status === 0) {
          message = 'Network error - please check your connection';
        } else {
          message = `HTTP ${error.status}: ${error.statusText}`;
        }

        this.errorService.showError(message);
        return throwError(() => error);
      })
    );
  }
}