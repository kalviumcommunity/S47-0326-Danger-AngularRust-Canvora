import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-tool-button',
  standalone: true,
  template: `
    <button class="tool-button" [disabled]="disabled" (click)="onClick()">
      {{ label }}
    </button>
  `,
  styles: [
    `
      .tool-button {
        display: block;
        width: 100%;
        padding: 0.5rem;
        margin-bottom: 0.5rem;
        border: 1px solid #999;
        border-radius: 5px;
        background-color: #fff;
        color: #111;
        cursor: pointer;
      }
      .tool-button:hover { background-color: #f0f0f0; }
      .tool-button:disabled { opacity: 0.5; cursor: not-allowed; }
    `
  ]
})
export class ToolButtonComponent {
  @Input() label = 'Button';
  @Input() disabled = false;
  @Input() action: () => void = () => {};

  onClick() {
    if (this.action) {
      this.action();
    }
  }
}
