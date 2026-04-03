import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected readonly title = signal('frontend');

  private drawing = false;
  private lastX = 0;
  private lastY = 0;
  private canvas?: HTMLCanvasElement;
  private ctx?: CanvasRenderingContext2D;

  ngAfterViewInit() {
    this.canvas = document.getElementById('whiteboard') as HTMLCanvasElement;
    if (!this.canvas) return;
    this.ctx = this.canvas.getContext('2d')!;
    this.canvas.addEventListener('mousedown', this.startDraw);
    this.canvas.addEventListener('mousemove', this.draw);
    this.canvas.addEventListener('mouseup', this.endDraw);
    this.canvas.addEventListener('mouseleave', this.endDraw);
    // Touch support for mobile
    this.canvas.addEventListener('touchstart', this.startDraw);
    this.canvas.addEventListener('touchmove', this.draw);
    this.canvas.addEventListener('touchend', this.endDraw);
    this.canvas.addEventListener('touchcancel', this.endDraw);
  }

  private getPos(e: MouseEvent | TouchEvent) {
    let x = 0, y = 0;
    if (e instanceof MouseEvent) {
      x = e.offsetX;
      y = e.offsetY;
    } else if (e.touches && e.touches.length > 0) {
      const rect = (e.target as HTMLCanvasElement).getBoundingClientRect();
      x = e.touches[0].clientX - rect.left;
      y = e.touches[0].clientY - rect.top;
    }
    return { x, y };
  }

  private startDraw = (e: MouseEvent | TouchEvent) => {
    e.preventDefault();
    this.drawing = true;
    const { x, y } = this.getPos(e);
    this.lastX = x;
    this.lastY = y;
  };

  private draw = (e: MouseEvent | TouchEvent) => {
    if (!this.drawing || !this.ctx) return;
    e.preventDefault();
    const { x, y } = this.getPos(e);
    this.ctx.beginPath();
    this.ctx.moveTo(this.lastX, this.lastY);
    this.ctx.lineTo(x, y);
    this.ctx.strokeStyle = '#222';
    this.ctx.lineWidth = 2;
    this.ctx.lineCap = 'round';
    this.ctx.stroke();
    this.lastX = x;
    this.lastY = y;
  };

  private endDraw = (e: MouseEvent | TouchEvent) => {
    e.preventDefault();
    this.drawing = false;
  };
}
