# Issue #10: Implement Strict TypeScript Models in Whiteboard UI

## Goal
Define and integrate strict frontend models for drawing data so the app is type-safe and easier to extend.

## Added model file
- `frontend/src/app/models/draw-models.ts`
  - `DrawPoint`
  - `DrawSegment`

## Whiteboard updates
- `WhiteboardComponent` now uses `DrawSegment` for local capture
- stroke points are stored and aggregated in `strokes: DrawSegment[]`
- `currentStroke` is typed and validated

## Benefits
- clear contract for real-time message payloads
- ready for WebSocket serialization and broadcast
- more maintainable code around drawing operations
