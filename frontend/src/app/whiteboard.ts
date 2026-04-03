import { AfterViewInit, Component, ElementRef, ViewChild } from '@angular/core';
import { DrawPoint, DrawSegment } from './models/draw-models';

@Component({
  selector: 'app-whiteboard',
  standalone: true,
  template: `
    <div class="whiteboard-app">
      <aside class="toolbar">
        <h3>Tools</h3>
        <button (click)="clearCanvas()">Clear</button>
        <p>Status: {{ drawing ? 'Drawing...' : 'Ready' }}</p>
      </aside>
      <section class="canvas-section">
        <h2>Whiteboard</h2>
        <canvas #canvas width="900" height="520" class="whiteboard-canvas"></canvas>
      </section>
    </div>
  `,
  styles: [
    `
      .whiteboard-app {
        display: grid;
        grid-template-columns: 200px 1fr;
        gap: 1rem;
        padding: 1rem;
        min-height: calc(100vh - 60px);
      }

      .toolbar {
        background: #f5f5f5;
        border: 1px solid #ccc;
        border-radius: 8px;
        padding: 1rem;
      }

      .canvas-section {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
      }

      .whiteboard-canvas {
        width: 100%;
        height: 520px;
        border: 1px solid #aaa;
        background: #fff;
        touch-action: none;
      }

      button {
        padding: 0.5rem 0.8rem;
        font-size: 0.9rem;
      }
    `
  ]
})
export class WhiteboardComponent implements AfterViewInit {
  @ViewChild('canvas', { static: true }) canvasRef!: ElementRef<HTMLCanvasElement>;

  private ctx!: CanvasRenderingContext2D;
  drawing = false;
  private lastX = 0;
  private lastY = 0;
  private currentStroke: DrawSegment | null = null;
  private strokes: DrawSegment[] = [];

  ngAfterViewInit() {
    const canvas = this.canvasRef.nativeElement;
    const context = canvas.getContext('2d');
    if (!context) {
      return;
    }

    this.ctx = context;

    window.addEventListener('resize', this.resizeCanvas);
    this.resizeCanvas();

    canvas.addEventListener('pointerdown', this.startDraw);
    canvas.addEventListener('pointermove', this.draw);
    canvas.addEventListener('pointerup', this.endDraw);
    canvas.addEventListener('pointercancel', this.endDraw);
    canvas.addEventListener('pointerleave', this.endDraw);
  }

  private startDraw = (event: PointerEvent) => {
    event.preventDefault();
    this.drawing = true;

    const canvas = this.canvasRef.nativeElement;
    canvas.setPointerCapture(event.pointerId);

    const startPoint: DrawPoint = { x: event.offsetX, y: event.offsetY };
    this.currentStroke = {
      id: crypto.randomUUID(),
      userId: 'local',
      points: [startPoint],
      color: '#333',
      width: 2
    };

    this.lastX = startPoint.x;
    this.lastY = startPoint.y;
  };

  private draw = (event: PointerEvent) => {
    if (!this.drawing) {
      return;
    }
    event.preventDefault();

    const x = event.offsetX;
    const y = event.offsetY;

    this.ctx.beginPath();
    this.ctx.moveTo(this.lastX, this.lastY);
    this.ctx.lineTo(x, y);
    this.ctx.strokeStyle = '#333';
    this.ctx.lineWidth = 2;
    this.ctx.lineCap = 'round';
    this.ctx.stroke();

    if (this.currentStroke) {
      this.currentStroke.points.push({ x, y });
    }

    this.lastX = x;
    this.lastY = y;
  };

  private endDraw = (event: PointerEvent) => {
    event.preventDefault();
    this.drawing = false;

    if (this.currentStroke) {
      this.strokes.push(this.currentStroke);
      this.currentStroke = null;
    }

    const canvas = this.canvasRef.nativeElement;
    if (canvas.hasPointerCapture(event.pointerId)) {
      canvas.releasePointerCapture(event.pointerId);
    }
  };

  clearCanvas() {
    const canvas = this.canvasRef.nativeElement;
    this.ctx.clearRect(0, 0, canvas.width, canvas.height);
    this.strokes = [];
  }

  private resizeCanvas = () => {
    const canvas = this.canvasRef.nativeElement;
    const newWidth = Math.max(window.innerWidth - 220, 800);
    const newHeight = Math.max(window.innerHeight - 200, 520);

    const imageData = this.ctx.getImageData(0, 0, canvas.width, canvas.height);
    canvas.width = newWidth;
    canvas.height = newHeight;
    this.ctx.putImageData(imageData, 0, 0);
  };;
}
