# Modern Web Architecture & WebSocket Topologies

## High-Level Architecture
- Angular SPA (frontend)
- Rust backend (REST + WebSocket)
- PostgreSQL database
- REST for CRUD, WebSocket for real-time sync

## WebSocket Topology
- Single server handles WebSocket connections
- Each whiteboard session = a "room"
- Server broadcasts updates to all clients in the same room
- Scalable to multi-server with pub/sub (e.g., Redis) in future

## Diagram (Mermaid)

```mermaid
graph TD
    A[Angular SPA] -- REST --> B(Rust Backend)
    A -- WebSocket --> B
    B -- DB Queries --> C[(PostgreSQL)]
    subgraph Room
        D[User 1]
        E[User 2]
        F[User N]
    end
    D -- WebSocket --> B
    E -- WebSocket --> B
    F -- WebSocket --> B
```

---

This document defines the core architecture and WebSocket topology for the MVP.
