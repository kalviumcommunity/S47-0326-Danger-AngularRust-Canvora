# Issue #15: Implement Responsive Infinite Canvas Mathematics

## Goal
Add responsive canvas resizing and baseline infinite canvas behavior.

## Implementation
- `whiteboard.ts` listens to window resize and adjusts canvas width/height.
- `resizeCanvas()` preserves current image data and keeps minimum dimensions.
- canvas area dynamically grows with viewport.

## Result
- core math behavior for large area implemented.
