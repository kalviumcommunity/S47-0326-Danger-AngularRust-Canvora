import { Injectable, Injector } from '@angular/core';
import {
  HttpInterceptor,
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpErrorResponse
} from '@angular/common/http';
import { Router } from '@angular/router';
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { AuthService } from './auth.service';
import { isApiRequestUrl } from './api-config';

/**
 * Attaches Bearer tokens only to requests targeting the configured API origin.
 * Uses Injector to resolve AuthService lazily and avoid HttpClient circular DI.
 */
@Injectable()
export class AuthInterceptor implements HttpInterceptor {
  constructor(
    private readonly injector: Injector,
    private readonly router: Router
  ) {}

  intercept(request: HttpRequest<unknown>, next: HttpHandler): Observable<HttpEvent<unknown>> {
    if (!isApiRequestUrl(request.url)) {
      return next.handle(request);
    }

    const auth = this.injector.get(AuthService);
    const token = auth.getToken();

    const authRequest =
      token && !auth.isTokenExpired()
        ? request.clone({
            setHeaders: { Authorization: `Bearer ${token}` }
          })
        : request;

    return next.handle(authRequest).pipe(
      catchError((error: HttpErrorResponse) => {
        if (error.status === 401) {
          auth.logout();
          void this.router.navigate(['/login'], { queryParams: { reason: 'session' } });
        }
        return throwError(() => error);
      })
    );
  }
}
