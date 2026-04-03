import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private isAuthenticatedSubject = new BehaviorSubject<boolean>(false);
  public isAuthenticated$ = this.isAuthenticatedSubject.asObservable();

  constructor() {
    this.checkAuth();
  }

  login(email: string, password: string): boolean {
    // Simulate authentication - in real app, call API
    if (email && password) {
      localStorage.setItem('authenticated', 'true');
      localStorage.setItem('user', email);
      this.isAuthenticatedSubject.next(true);
      return true;
    }
    return false;
  }

  logout(): void {
    localStorage.removeItem('authenticated');
    localStorage.removeItem('user');
    this.isAuthenticatedSubject.next(false);
  }

  private checkAuth(): void {
    const auth = localStorage.getItem('authenticated') === 'true';
    this.isAuthenticatedSubject.next(auth);
  }

  get isAuthenticated(): boolean {
    return this.isAuthenticatedSubject.value;
  }

  get currentUser(): string | null {
    return localStorage.getItem('user');
  }
}