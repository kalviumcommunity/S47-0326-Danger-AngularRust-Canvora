import { Component } from '@angular/core';
import { ReactiveFormsModule, FormGroup, FormControl, Validators } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [ReactiveFormsModule],
  template: `
    <div style="display:flex;flex-direction:column;align-items:center;gap:10px;max-width:300px;margin:0 auto;">
      <h2>Login</h2>
      <form [formGroup]="loginForm" (ngSubmit)="onSubmit()">
        <div>
          <label for="username">Username:</label>
          <input id="username" type="text" formControlName="username">
        </div>
        <div>
          <label for="password">Password:</label>
          <input id="password" type="password" formControlName="password">
        </div>
        <button type="submit" [disabled]="loginForm.invalid">Login</button>
      </form>
    </div>
  `
})
export class LoginComponent {
  loginForm = new FormGroup({
    username: new FormControl('', Validators.required),
    password: new FormControl('', Validators.required)
  });

  constructor(private router: Router) {}

  onSubmit() {
    if (this.loginForm.valid) {
      // Simulate authentication
      localStorage.setItem('authenticated', 'true');
      this.router.navigate(['/whiteboard']);
    }
  }
}