import { Component } from '@angular/core';

@Component({
  selector: 'app-login',
  standalone: true,
  template: `
    <div style="display:flex;flex-direction:column;align-items:center;gap:10px;">
      <h2>Login (simulation)</h2>
      <p>This is a placeholder login page; auto-allow</p>
      <a routerLink="/whiteboard">Go to Whiteboard</a>
    </div>
  `
})
export class LoginComponent {}