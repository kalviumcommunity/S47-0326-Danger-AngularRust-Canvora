import { ApplicationConfig, ErrorHandler, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter, withPreloading } from '@angular/router';
import { provideHttpClient, withInterceptorsFromDi, HTTP_INTERCEPTORS } from '@angular/common/http';

import { routes } from './app.routes';
import { GlobalErrorHandler } from './global-error-handler';
import { LoadingInterceptor } from './loading.interceptor';
import { ErrorInterceptor } from './error.interceptor';
import { AuthInterceptor } from './auth.interceptor';
import { SelectivePreloadingStrategy } from './selective-preloading.strategy';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes, withPreloading(SelectivePreloadingStrategy)),
    provideHttpClient(withInterceptorsFromDi()),
    { provide: HTTP_INTERCEPTORS, useClass: AuthInterceptor, multi: true },
    { provide: HTTP_INTERCEPTORS, useClass: LoadingInterceptor, multi: true },
    { provide: HTTP_INTERCEPTORS, useClass: ErrorInterceptor, multi: true },
    {
      provide: ErrorHandler,
      useClass: GlobalErrorHandler
    }
  ]
};
