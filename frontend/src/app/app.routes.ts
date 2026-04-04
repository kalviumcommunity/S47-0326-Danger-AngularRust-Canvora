import { Routes } from '@angular/router';

export const routes: Routes = [
  { path: '', redirectTo: 'whiteboard', pathMatch: 'full' },
  {
    path: 'login',
    loadChildren: () => import('./login.module').then(m => m.LoginModule)
  },
  {
    path: 'whiteboard',
    loadChildren: () => import('./whiteboard.module').then(m => m.WhiteboardModule)
  },
  { path: '**', redirectTo: 'whiteboard' },
];
