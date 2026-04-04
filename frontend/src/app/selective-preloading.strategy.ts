import { Injectable } from '@angular/core';
import { PreloadingStrategy, Route } from '@angular/router';
import { Observable, EMPTY } from 'rxjs';

/**
 * Preloads only routes marked with `data: { preload: true }` so heavy workspace
 * chunks stay out of the initial navigation path unless explicitly opted in.
 */
@Injectable({ providedIn: 'root' })
export class SelectivePreloadingStrategy implements PreloadingStrategy {
  preload(route: Route, load: () => Observable<unknown>): Observable<unknown> {
    if (route.data?.['preload']) {
      return load();
    }
    return EMPTY;
  }
}
