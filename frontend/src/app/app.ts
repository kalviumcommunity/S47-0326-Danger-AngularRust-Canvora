import { Component, signal } from '@angular/core';
import { WhiteboardComponent } from './whiteboard';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [WhiteboardComponent],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected readonly title = signal('frontend');
}
