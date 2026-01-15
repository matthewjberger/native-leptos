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
- Node.js (for Tailwind CSS in component-gallery)

### Running the Desktop App

```bash
just run
```

### Running the Component Gallery

```bash
cd component-gallery
just setup    # First time only
just serve
```

The gallery will be available at http://localhost:8080

### Running the Site (standalone)

```bash
cd site
just serve
```

### Running the API Server

```bash
cd api
cargo run
```

## UI Component Library

The `ui/` crate provides shared components:

- **Buttons**: Button, IconButton, NavButton, SelectButton
- **Icons**: CheckIcon, CloseIcon, BackArrowIcon, MenuIcon, etc.
- **Dialog**: Modal dialogs (Info, Warning, Danger)
- **Toast**: Notifications (Success, Warning, Info, Error)
- **Toggle**: Toggle switches and ToggleGroup

### Usage

```rust
use ui::*;

view! {
    <Button variant=ButtonVariant::Primary>"Click me"</Button>
    <Dialog visible=show_dialog dialog_type=Signal::derive(|| DialogType::Warning)>
        <p>"Warning message"</p>
    </Dialog>
}
```

## API Server

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/api/v1/resources` | List resources |
| POST | `/api/v1/resources` | Create resource |
| GET | `/api/v1/resources/{id}` | Get resource |
| PUT | `/api/v1/resources/{id}` | Update resource |
| DELETE | `/api/v1/resources/{id}` | Delete resource |

### Configuration

- `BIND_ADDRESS` - Server address (default: `0.0.0.0:8080`)
- `DATABASE_URL` - SQLite path or Postgres connection string
- `AUTH_ENABLED` - Enable auth middleware (default: `false`)

## License

Dual-licensed under MIT or Apache-2.0.
