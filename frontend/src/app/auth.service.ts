import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { HttpClient } from '@angular/common/http';

interface LoginResponse {
  user: { id: string; email: string; name: string };
  token: string;
  expires_at: number;
}

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private isAuthenticatedSubject = new BehaviorSubject<boolean>(false);
  private currentUserSubject = new BehaviorSubject<any>(null);
  public isAuthenticated$ = this.isAuthenticatedSubject.asObservable();
  public currentUser$ = this.currentUserSubject.asObservable();

  private readonly TOKEN_KEY = 'auth_token';
  private readonly USER_KEY = 'auth_user';
  private readonly EXPIRES_KEY = 'auth_expires';

  constructor(private http: HttpClient) {
    this.checkAuth();
  }

  login(email: string, password: string): Observable<LoginResponse> {
    return this.http.post<LoginResponse>('http://127.0.0.1:8080/login', { email, password });
  }

  setAuthData(response: LoginResponse): void {
    localStorage.setItem(this.TOKEN_KEY, response.token);
    localStorage.setItem(this.USER_KEY, JSON.stringify(response.user));
    localStorage.setItem(this.EXPIRES_KEY, response.expires_at.toString());
    this.isAuthenticatedSubject.next(true);
    this.currentUserSubject.next(response.user);
  }

  logout(): void {
    localStorage.removeItem(this.TOKEN_KEY);
    localStorage.removeItem(this.USER_KEY);
    localStorage.removeItem(this.EXPIRES_KEY);
    this.isAuthenticatedSubject.next(false);
    this.currentUserSubject.next(null);
  }

  private checkAuth(): void {
    const token = this.getToken();
    const expiresAt = this.getExpiresAt();

    if (token && expiresAt && Date.now() < expiresAt * 1000) {
      const user = this.getUser();
      this.isAuthenticatedSubject.next(true);
      this.currentUserSubject.next(user);
    } else {
      this.logout();
    }
  }

  getToken(): string | null {
    return localStorage.getItem(this.TOKEN_KEY);
  }

  private getUser(): any {
    const userStr = localStorage.getItem(this.USER_KEY);
    return userStr ? JSON.parse(userStr) : null;
  }

  private getExpiresAt(): number | null {
    const expiresStr = localStorage.getItem(this.EXPIRES_KEY);
    return expiresStr ? parseInt(expiresStr, 10) : null;
  }

  get isAuthenticated(): boolean {
    return this.isAuthenticatedSubject.value;
  }

  get currentUser(): any {
    return this.currentUserSubject.value;
  }

  isTokenExpired(): boolean {
    const expiresAt = this.getExpiresAt();
    return !expiresAt || Date.now() >= expiresAt * 1000;
  }
}