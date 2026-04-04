import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { ReactiveFormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { LoginComponent } from './login';

@NgModule({
  imports: [CommonModule, ReactiveFormsModule, LoginComponent, RouterModule.forChild([{ path: '', component: LoginComponent }])]
})
export class LoginModule {}
