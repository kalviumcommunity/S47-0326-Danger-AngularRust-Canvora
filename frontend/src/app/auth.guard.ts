import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';

export const authGuard: CanActivateFn = () => {
  const router = inject(Router);
  const isAuthenticated = true; // replace with real auth state

  if (!isAuthenticated) {
    router.navigate(['/login']);
    return false;
  }

  return true;
};