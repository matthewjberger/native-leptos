# native-leptos

A native desktop application that hosts a WebView with bidirectional IPC communication to a Rust/WASM frontend.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Native App (Rust)                     │
│  ┌───────────────┐    ┌──────────────────────────────┐  │
│  │   Nightshade  │    │      WebView (wry)           │  │
│  │    (egui)     │    │  ┌────────────────────────┐  │  │
│  │               │    │  │   Leptos WASM App      │  │  │
│  │               │◄───┼──│                        │  │  │
│  │               │    │  │   postcard + base64    │  │  │
│  │               │────┼──►                        │  │  │
│  │               │    │  └────────────────────────┘  │  │
│  └───────────────┘    └──────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Structure

- `src/` - Native Rust backend
- `protocol/` - Shared message types (used by both native and WASM)
- `leptos-site/` - Leptos frontend compiled to WASM

## Communication

Bidirectional IPC using:
- **Serialization**: postcard (binary, compact)
- **Transport**: base64 over wry's native IPC (faster than WebSockets)

### Message Types

```rust
// Frontend → Backend
FrontendCommand::Ready
FrontendCommand::RequestRandomNumber { request_id }

// Backend → Frontend
BackendEvent::Connected
BackendEvent::RandomNumber { request_id, value }
```

## Building

```bash
# Build the frontend (WASM)
just build-frontend

# Build and run the app
just run
```

## Requirements

- Rust 1.90+
- [Trunk](https://trunkrs.dev/) for building the WASM frontend

## License

MIT OR Apache-2.0
