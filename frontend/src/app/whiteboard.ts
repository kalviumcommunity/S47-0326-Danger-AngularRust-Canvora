import { AfterViewInit, Component, ElementRef, ViewChild } from '@angular/core';
import { DrawPoint, DrawSegment } from './models/draw-models';

@Component({
  selector: 'app-whiteboard',
  standalone: true,
  template: `
    <section class="whiteboard-shell">
      <h2>Whiteboard</h2>
      <canvas #canvas width="900" height="520" class="whiteboard-canvas"></canvas>
    </section>
  `,
  styles: [
    `
      .whiteboard-shell {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 1rem;
      }
      .whiteboard-canvas {
        border: 1px solid #aaa;
        background: #fff;
        touch-action: none;
      }
    `
  ]
})
export class WhiteboardComponent implements AfterViewInit {
  @ViewChild('canvas', { static: true }) canvasRef!: ElementRef<HTMLCanvasElement>;

  private ctx!: CanvasRenderingContext2D;
  private drawing = false;
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
}
