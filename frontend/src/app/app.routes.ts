import { Routes } from '@angular/router';
import { WhiteboardComponent } from './whiteboard';
import { LoginComponent } from './login';
import { authGuard } from './auth.guard';

export const routes: Routes = [
  { path: '', redirectTo: 'whiteboard', pathMatch: 'full' },
  { path: 'login', component: LoginComponent },
  {
    path: 'whiteboard',
    component: WhiteboardComponent,
    canActivate: [authGuard],
  },
  { path: '**', redirectTo: 'whiteboard' },
];
