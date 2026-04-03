# Issue #7: Low-Level Design (LLD)

## Purpose
Detail concrete architecture and implementation points for the real-time whiteboard features.

## Frontend Angular LLD
- Root app uses standalone component.
- `WhiteboardComponent` 
  - uses `<canvas>` and `pointer` events.
  - stores all drawn strokes in memory as `DrawSegment` objects.
  - connects to `WebsocketService` (future) for send/receive draw actions.
- `WebsocketService` (not implemented yet) will:
  - connect to `ws://localhost:8080/ws`
  - emit redraw messages to active subscribers
  - handle inbound broadcast messages from server

### Models
- `DrawSegment`
  - `id: string`
  - `userId: string`
  - `points: Array<{x:number,y:number}>`
  - `color: string`
  - `width: number`

## Backend Rust LLD
- `main.rs` sets up HTTP + WebSocket route.
- Store active sessions in `Arc<Mutex<HashMap<Uuid, Sender<Message>>>>`.
- Accept JSON commands:
  - `draw` (DrawSegment payload)
  - forwarding to all connected clients.

## Data contract JSON (protobuf-light)
- outbound inbound messages:
  ```json
  {
    "type": "draw",
    "payload": {
      "id": "uuid",
      "userId": "user-123",
      "points": [{"x":0,"y":0},{"x":10,"y":10}],
      "color":"#000000",
      "width":2
    }
  }
  ```

## Non-goals
- CRUD endpoints
- DB persistence
- security/auth
