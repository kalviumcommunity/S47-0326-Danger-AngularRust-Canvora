# Issue #11: Architect Rust Ownership and Concurrency Model

## Goal
Introduce explicit shared-state and concurrency model in backend for collaborative whiteboard.

## Implementation
- `AppState` wraps `board` in `Arc<Mutex<Vec<DrawSegment>>>`
- `DrawSegment`, `DrawPoint` structs are `Serialize`/`Deserialize`
- `GET /state` returns board state
- `POST /draw` appends segment thread-safely

## Notes
- This is a minimal and safe shared-memory model.
- Scalability later: replace with actor or broadcasting system.
