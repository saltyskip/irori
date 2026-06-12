# Hearth

A shared hub for your memories and collections. Built in Rust with Axum, PostgreSQL, and a protocol-based sync system.

## Vision

Hearth is a extensible platform for shared repositories of any kind. v1 focuses on family photo libraries, but the architecture is designed to support music libraries, document vaults, and more.

**Key features:**
- 🔄 Protocol-based sync (pluggable implementations)
- 👥 Family-first sharing with roles and permissions
- 📦 Flexible storage (NAS, local, S3-compatible)
- 🚀 Built for performance in Rust
- 🔌 Extensible service layer via traits

## Architecture

```
hearth-rust/
├── server/           # Axum API + business logic
│   ├── src/
│   │   ├── api/      # HTTP transport (thin wrappers)
│   │   ├── mcp/      # MCP protocol (Claude integration)
│   │   ├── services/ # Business logic (transport-agnostic)
│   │   └── core/     # Database, storage, config
│   └── migrations/   # SQL schemas
├── cli/              # Desktop/mobile client
└── Cargo.toml        # Workspace root
```

### Service Layer (Transport-Agnostic)

- **resources** — Upload, download, list files/photos/documents
- **collections** — Organize resources into albums, playlists, folders
- **sharing** — Invite members, assign roles (owner/editor/viewer)
- **sync** — Protocol abstraction (Hearth protocol + Immich compatibility)
- **users** — Registration, authentication, profiles

### Transport Layers

- **HTTP API** (`src/api/`) — REST endpoints
- **MCP** (`src/mcp/`) — Claude/AI integration
- Both import from `services/`, never from each other

## Setup

### Prerequisites

- Rust 1.81+
- PostgreSQL 14+
- OrbStack or Docker

### Development

```bash
# Clone
cd /Users/andreiterentiev/Developer/hearth-rust

# Start database + server with OrbStack
orbstack start
docker-compose up

# In another terminal, run migrations
cargo run -- migrate --name m001_init_schema --apply

# Start server
cargo run -- serve
```

Server will be available at `http://localhost:3000`

Health check: `curl http://localhost:3000/health`

### Configuration

Set environment variables (or use `.env`):

```bash
DATABASE_URL=postgres://hearth:hearth-dev@localhost/hearth
STORAGE_BACKEND=local
STORAGE_PATH=/tmp/hearth-storage
NAS_MOUNT_PATH=/mnt/immich-nas (optional)
SYNC_PROTOCOL=hearth (or "immich" for compatibility)
JWT_SECRET=dev-secret-change-in-production
```

## CLI

```bash
# Initialize client
hearth init --server http://localhost:3000 --email alice@family.com

# Watch directory for sync
hearth watch ~/Pictures --collection "Family Photos"

# List collections
hearth list

# Invite family member
hearth invite bob@family.com --collection "Summer 2024" --role viewer

# Show sync status
hearth status
```

## API Documentation

Once the server is running, visit `http://localhost:3000/docs` for interactive OpenAPI docs.

## Sync Protocol

Hearth uses a pluggable sync protocol. The default implementation is designed for efficient multi-device sync:

- **Identify** — Client introduces itself
- **Fetch changes** — Get delta since last sync (cursor-based)
- **Push changes** — Send local edits, get conflict info back
- **Upload/download files** — Transfer media

See `PROTOCOL.md` for detailed spec.

### Protocol Implementations

- **Hearth** (default) — Optimized for multi-device family use
- **Immich** — Compatible with Immich's sync protocol (allows migration)

## NAS Integration

For UNASPro or other NFS-mounted storage:

```bash
# Mount UNASPro
sudo mount -t nfs 192.168.1.100:/mnt/immich /mnt/immich-nas

# Set in docker-compose.yml or env
NAS_MOUNT_PATH=/mnt/immich-nas
STORAGE_BACKEND=nas
```

The storage backend is pluggable (`core/storage.rs`). Current implementations:

- `LocalStorage` — Filesystem (development)
- `NasStorage` — NFS mount (production)
- `S3Storage` — Coming soon

## Testing

```bash
# Run all tests
cargo test

# Run with backtrace on failure
RUST_BACKTRACE=1 cargo test

# Run specific test
cargo test health::
```

## Development Guidelines

See `CLAUDE.md` for:
- Architecture decisions
- Service layer patterns
- Testing conventions
- Git workflows

## License

Dual licensed under MIT OR Apache-2.0
