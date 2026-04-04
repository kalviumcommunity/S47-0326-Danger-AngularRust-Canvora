import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { WhiteboardComponent } from './whiteboard';
import { ToolButtonComponent } from './tool-button';
import { authGuard } from './auth.guard';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    ToolButtonComponent,
    WhiteboardComponent,
    RouterModule.forChild([
      {
        path: '',
        component: WhiteboardComponent,
        canActivate: [authGuard]
      }
    ])
  ]
})
export class WhiteboardModule {}
