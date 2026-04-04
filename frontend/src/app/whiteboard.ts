import { AfterViewInit, Component, ElementRef, ViewChild, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ToolButtonComponent } from './tool-button';
import { ApiService } from './api.service';
import { Board, CreateBoardRequest } from './models/board-models';
import { DrawPoint, DrawSegment } from './models/draw-models';
import { WhiteboardStateService } from './whiteboard-state.service';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-whiteboard',
  standalone: true,
  imports: [CommonModule, FormsModule, ToolButtonComponent],
  template: `
    <div class="whiteboard-app">
      <aside class="toolbar">
        <h3>Tools</h3>

        <div class="board-shell">
          <label>
            Board:
            <select [(ngModel)]="selectedBoardId" (ngModelChange)="onBoardSelected($event)">
              <option *ngFor="let board of boards" [value]="board.id">{{ board.name }}</option>
            </select>
          </label>
          <div class="board-creator">
            <input type="text" placeholder="New board name" [(ngModel)]="boardName" />
            <button type="button" (click)="createNewBoard()">Create</button>
          </div>
        </div>

        <p class="current-board">Current board: {{ currentBoard?.name || 'None' }}</p>
        <p *ngIf="errorMessage" class="error">{{ errorMessage }}</p>

        <app-tool-button label="Clear" [action]="clearCanvas"></app-tool-button>
        <app-tool-button label="Reset" [action]="resetBoard"></app-tool-button>
        <label>Color: <input type="color" [(ngModel)]="penColor" (ngModelChange)="onColorChange($event)" /></label>
        <label>Width: <input type="range" min="1" max="10" [(ngModel)]="penWidth" (ngModelChange)="onWidthChange($event)" /></label>
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
export class WhiteboardComponent implements AfterViewInit, OnDestroy {
  @ViewChild('canvas', { static: true }) canvasRef!: ElementRef<HTMLCanvasElement>;

  private ctx!: CanvasRenderingContext2D;
  drawing = false;
  penColor = '#333333';
  penWidth = 2;
  private lastX = 0;
  private lastY = 0;
  private currentStroke: DrawSegment | null = null;
  private subscriptions: Subscription[] = [];
  boards: Board[] = [];
  currentBoard: Board | null = null;
  selectedBoardId = '';
  boardName = 'New board';
  errorMessage = '';

  constructor(
    private whiteboardState: WhiteboardStateService,
    private api: ApiService
  ) {}

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

    this.loadBoards();

    // Subscribe to state changes
    this.subscriptions.push(
      this.whiteboardState.segments$.subscribe(segments => {
        this.redrawCanvas(segments);
      })
    );

    this.subscriptions.push(
      this.whiteboardState.penSettings$.subscribe(settings => {
        this.penColor = settings.color;
        this.penWidth = settings.width;
      })
    );

    // Initialize from service
    this.penColor = this.whiteboardState.penSettings.color;
    this.penWidth = this.whiteboardState.penSettings.width;
  }

  ngOnDestroy() {
    this.subscriptions.forEach(sub => sub.unsubscribe());
    window.removeEventListener('resize', this.resizeCanvas);
  }

  private startDraw = (event: PointerEvent) => {
    event.preventDefault();
    this.drawing = true;

    const canvas = this.canvasRef.nativeElement;
    canvas.setPointerCapture(event.pointerId);

    const startPoint: DrawPoint = { x: event.offsetX, y: event.offsetY };
    const boardId = this.currentBoard?.id ?? 'local';

    this.currentStroke = {
      id: crypto.randomUUID(),
      board_id: boardId,
      user_id: 'local',
      points: [startPoint],
      color: this.penColor,
      width: this.penWidth,
      created_at: Date.now()
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
    this.ctx.strokeStyle = this.penColor;
    this.ctx.lineWidth = this.penWidth;
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
      this.whiteboardState.addSegment(this.currentStroke);

      if (this.currentBoard) {
        this.api.saveDrawingSegment(this.currentStroke).subscribe({
          next: () => {},
          error: (err) => console.error('Failed to save drawing segment', err)
        });
      }

      this.currentStroke = null;
    }

    const canvas = this.canvasRef.nativeElement;
    if (canvas.hasPointerCapture(event.pointerId)) {
      canvas.releasePointerCapture(event.pointerId);
    }
  };

  clearCanvas = () => {
    this.whiteboardState.clearSegments();
  };

  resetBoard = () => {
    this.whiteboardState.updatePenSettings({ color: '#333333', width: 2 });
    this.whiteboardState.clearSegments();
  };

  onColorChange(color: string) {
    this.whiteboardState.updatePenSettings({ color });
  }

  onWidthChange(width: number) {
    this.whiteboardState.updatePenSettings({ width });
  }

  private redrawCanvas(segments: DrawSegment[]) {
    const canvas = this.canvasRef.nativeElement;
    this.ctx.clearRect(0, 0, canvas.width, canvas.height);

    segments.forEach(segment => {
      if (segment.points.length < 2) return;

      this.ctx.beginPath();
      this.ctx.strokeStyle = segment.color;
      this.ctx.lineWidth = segment.width;
      this.ctx.lineCap = 'round';

      this.ctx.moveTo(segment.points[0].x, segment.points[0].y);
      for (let i = 1; i < segment.points.length; i++) {
        this.ctx.lineTo(segment.points[i].x, segment.points[i].y);
      }
      this.ctx.stroke();
    });
  }

  private resizeCanvas = () => {
    const canvas = this.canvasRef.nativeElement;
    const newWidth = Math.max(window.innerWidth - 220, 800);
    const newHeight = Math.max(window.innerHeight - 200, 520);

    const imageData = this.ctx.getImageData(0, 0, canvas.width, canvas.height);
    canvas.width = newWidth;
    canvas.height = newHeight;
    this.ctx.putImageData(imageData, 0, 0);
  };

  private loadBoards() {
    this.api.getBoards().subscribe({
      next: (boardPage) => {
        this.boards = boardPage.items;
        if (this.boards.length > 0 && !this.currentBoard) {
          this.selectedBoardId = this.boards[0].id;
          this.onBoardSelected(this.boards[0].id);
        }
      },
      error: (err) => {
        console.error('Failed to load boards', err);
        this.errorMessage = 'Failed to load boards';
      }
    });
  }

  onBoardSelected(boardId: string) {
    const board = this.boards.find(b => b.id === boardId);
    if (board) {
      this.currentBoard = board;
      this.errorMessage = '';
      // Load drawings for this board
      this.api.getBoardDrawings(boardId).subscribe({
        next: (drawings) => {
          this.whiteboardState.setSegments(drawings);
        },
        error: (err) => {
          console.error('Failed to load drawings', err);
          this.errorMessage = 'Failed to load drawings';
        }
      });
    }
  }

  createNewBoard() {
    if (!this.boardName.trim()) {
      this.errorMessage = 'Board name cannot be empty';
      return;
    }

    const request: CreateBoardRequest = {
      name: this.boardName.trim(),
      is_public: false
    };

    this.api.createBoard(request).subscribe({
      next: (board) => {
        this.boards.push(board);
        this.selectedBoardId = board.id;
        this.onBoardSelected(board.id);
        this.boardName = 'New board';
        this.errorMessage = '';
      },
      error: (err) => {
        console.error('Failed to create board', err);
        this.errorMessage = 'Failed to create board';
      }
    });
  }
}
