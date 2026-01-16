# native-leptos

A native desktop application that hosts a WebView with bidirectional IPC communication to a Rust/WASM frontend, with an optional REST API server for cloud deployment.

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

## Project Structure

This is a Cargo workspace with all crates sharing a single `Cargo.lock`.

```
native-leptos/
├── src/                    # Native desktop app (Nightshade + wry)
├── site/                   # Leptos frontend (WASM)
├── ui/                     # Shared UI component library
├── component-gallery/      # Storybook-style component showcase
├── protocol/               # Shared IPC message types
├── api/                    # Axum REST API server
└── api-types/              # Shared API types
```

## Quick Start

### Prerequisites

- Rust 1.90+
- [Trunk](https://trunkrs.dev/) - `cargo install trunk`
- [just](https://github.com/casey/just) - `cargo install just`
- Node.js (for Tailwind CSS)

```bash
just setup  # Install dependencies
just run    # Run the desktop app
just --list # See all commands
```

## License

Dual-licensed under MIT or Apache-2.0.
