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

┌─────────────────────────────────────────────────────────┐
│                    API Server (Optional)                 │
│  ┌───────────────┐    ┌──────────────────────────────┐  │
│  │     Axum      │    │      SQLite / Postgres       │  │
│  │  REST API     │◄──►│      (via sqlx)              │  │
│  │               │    │                              │  │
│  │  Auth-ready   │    │  Local: SQLite file          │  │
│  │  middleware   │    │  Cloud: Postgres (RDS, etc)  │  │
│  └───────────────┘    └──────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Structure

- `src/` - Native Rust backend
- `protocol/` - Shared IPC message types (used by both native and WASM)
- `site/` - Leptos frontend compiled to WASM
- `api/` - Axum REST API server (deployable to AWS)
- `api-types/` - Shared API types (used by frontend and API server)

## Communication

Bidirectional IPC using:
- **Serialization**: postcard (binary, compact)
- **Transport**: base64 over wry's native IPC (faster than WebSockets)

## Building

```bash
# Build the frontend (WASM)
just build-frontend

# Build and run the app
just run
```

## API Server

The `api/` crate provides a REST API that can run locally or deploy to cloud infrastructure.

### Local Development

```bash
# Run with Docker
docker-compose up --build

# Or run directly
cd api && cargo run
```

The API server runs on `http://127.0.0.1:3000` by default.

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

Environment variables (see `api/.env.example`):
- `BIND_ADDRESS` - Server address (default: `0.0.0.0:8080`)
- `DATABASE_URL` - SQLite path or Postgres connection string
- `AUTH_ENABLED` - Enable auth middleware (default: `false`)

### Cloud Deployment

```bash
docker build -f api/Dockerfile -t native-leptos-api .
# Deploy to ECS/Fargate/App Runner
# Set DATABASE_URL to RDS Postgres
# Set AUTH_ENABLED=true when ready
```

## Requirements

- Rust 1.90+
- [Trunk](https://trunkrs.dev/) for building the WASM frontend
- Docker (optional, for running API locally)

## License

This project is free, open source and permissively licensed! All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
