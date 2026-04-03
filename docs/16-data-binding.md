# Issue #16: Implement Data Binding for Drawing & Interaction Events

## Goal
Bind tool settings to UI and use them in draw mechanics with model-driven controls.

## Added
- `penColor` and `penWidth` on `WhiteboardComponent`
- `ngModel` bindings in toolbar controls
- stroke rendering uses these bound properties

## Result
- direct UI controls that drive draw behavior
- foundation for additional dynamic property binding
