import { Component } from '@angular/core';
import { ReactiveFormsModule, FormGroup, FormControl, Validators } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { AuthService } from './auth.service';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [ReactiveFormsModule, CommonModule],
  template: `
    <div style="display:flex;flex-direction:column;align-items:center;gap:10px;max-width:300px;margin:0 auto;">
      <h2>Login</h2>
      <form [formGroup]="loginForm" (ngSubmit)="onSubmit()">
        <div>
          <label for="username">Email:</label>
          <input id="username" type="email" formControlName="username">
          <div *ngIf="loginForm.get('username')?.invalid && loginForm.get('username')?.touched" style="color:red;font-size:12px;">
            <div *ngIf="loginForm.get('username')?.errors?.['required']">Email is required</div>
            <div *ngIf="loginForm.get('username')?.errors?.['email']">Please enter a valid email</div>
          </div>
        </div>
        <div>
          <label for="password">Password:</label>
          <input id="password" type="password" formControlName="password">
          <div *ngIf="loginForm.get('password')?.invalid && loginForm.get('password')?.touched" style="color:red;font-size:12px;">
            <div *ngIf="loginForm.get('password')?.errors?.['required']">Password is required</div>
            <div *ngIf="loginForm.get('password')?.errors?.['minlength']">Password must be at least 8 characters</div>
            <div *ngIf="loginForm.get('password')?.errors?.['pattern']">Password must contain uppercase, lowercase, number, and special character</div>
          </div>
        </div>
        <button type="submit" [disabled]="loginForm.invalid">Login</button>
      </form>
    </div>
  `
})
export class LoginComponent {
  loginForm = new FormGroup({
    username: new FormControl('', [Validators.required, Validators.email]),
    password: new FormControl('', [
      Validators.required,
      Validators.minLength(8),
      Validators.pattern(/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]/)
    ])
  });

  constructor(private router: Router, private authService: AuthService) {}

  onSubmit() {
    if (this.loginForm.valid) {
      const success = this.authService.login(
        this.loginForm.value.username!,
        this.loginForm.value.password!
      );
      if (success) {
        this.router.navigate(['/whiteboard']);
      }
    }
  }
}