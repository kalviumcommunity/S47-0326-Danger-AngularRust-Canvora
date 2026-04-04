import { ApplicationConfig, ErrorHandler, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter } from '@angular/router';
import { provideHttpClient, withInterceptors } from '@angular/common/http';

import { routes } from './app.routes';
import { GlobalErrorHandler } from './global-error-handler';
import { LoadingInterceptor } from './loading.interceptor';
import { ErrorInterceptor } from './error.interceptor';
import { AuthInterceptor } from './auth.interceptor';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    provideHttpClient(
      withInterceptors([AuthInterceptor, LoadingInterceptor, ErrorInterceptor])
    ),
    {
      provide: ErrorHandler,
      useClass: GlobalErrorHandler
    }
  ]
};
