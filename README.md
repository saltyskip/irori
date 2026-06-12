# Irori 囲炉裏

*A shared flame for your memories and collections.*

Irori is a family-first photo and media repository, built in Rust. Named after the traditional Japanese sunken hearth—a gathering place where families shared stories and warmth. Like the irori itself, Irori is designed to be the center of your home: the place where memories converge.

Built with Axum, PostgreSQL, and a protocol-based sync system that respects your privacy and lets you sync across devices, on your own infrastructure.

## Vision

Irori is an extensible platform for shared repositories of any kind. v1 focuses on family photo libraries, but the architecture is designed to support music libraries, document vaults, family trees, and more.

**Core principles:**
- 👨‍👩‍👧‍👦 **Family-first** — Built for multi-generational sharing
- 🔄 **Protocol-based sync** — Pluggable implementations (native + Immich-compatible)
- 📦 **Flexible storage** — NAS (your UNASPro), local, or S3-compatible
- 🚀 **Performance** — Written in Rust for speed and safety
- 🔌 **Extensible** — Service layer via traits, swap implementations freely
- 🏠 **Self-hosted** — Keep your memories on your own infrastructure

## Architecture

```
irori/
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

- **resources** — Upload, download, list files/photos/videos/documents
- **collections** — Organize resources into albums, playlists, folders, galleries
- **sharing** — Invite family members, assign roles (owner/editor/viewer)
- **sync** — Protocol abstraction (Irori protocol + Immich compatibility)
- **users** — Registration, authentication, family profiles

### Transport Layers

- **HTTP API** (`src/api/`) — REST endpoints for web and mobile clients
- **MCP** (`src/mcp/`) — Claude/AI agent integration
- Both import from `services/`, never from each other

## Setup

### Prerequisites

- Rust 1.81+
- PostgreSQL 14+
- OrbStack or Docker
- (Optional) UNASPro or NFS-mounted NAS

### Development

```bash
# Clone
cd /Users/andreiterentiev/Developer/irori

# Start database + server with OrbStack
orbstack start
docker-compose up

# In another terminal, run migrations
cargo run -- migrate --name m001_init_schema --apply

# Start server (default: http://localhost:3000)
cargo run -- serve
```

Health check:
```bash
curl http://localhost:3000/health
```

### Configuration

Set environment variables (or use `.env`):

```bash
DATABASE_URL=postgres://irori:irori-dev@localhost/irori
STORAGE_BACKEND=local
STORAGE_PATH=/tmp/irori-storage
NAS_MOUNT_PATH=/mnt/immich-nas (optional)
SYNC_PROTOCOL=irori (or "immich" for compatibility)
JWT_SECRET=dev-secret-change-in-production
```

## CLI

```bash
# Initialize client
irori init --server http://localhost:3000 --email alice@family.com

# Watch directory for sync
irori watch ~/Pictures --collection "Family Photos"

# List collections
irori list

# Invite family member
irori invite bob@family.com --collection "Summer 2024" --role viewer

# Show sync status
irori status
```

## Sync Protocol

Irori uses a pluggable sync protocol optimized for multi-device family use:

- **Identify** — Client introduces itself
- **Fetch changes** — Get delta since last sync (cursor-based)
- **Push changes** — Send local edits, get conflict info back
- **Upload/download files** — Transfer media

See `PROTOCOL.md` for detailed spec.

### Protocol Implementations

- **Irori** (default) — Optimized for multi-device family use
- **Immich** — Compatible with Immich's sync protocol (allows data migration)

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
- `NasStorage` — NFS mount (production, your use case)
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

## Roadmap

- ✅ Service trait definitions
- ✅ Storage abstraction (NAS-first)
- ✅ Basic API scaffold
- ⏳ **Phase 1** — Resource upload/download, collections, basic sync
- ⏳ **Phase 2** — Multi-user sharing, family roles, MCP tools
- ⏳ **Phase 3** — Advanced search, tagging, timeline views
- ⏳ **Phase 4** — Mobile app (iOS/Android via Tauri or native)
- ⏳ **Phase 5** — Web UI (React)

## Why "Irori"?

囲炉裏 (irori) is a traditional Japanese sunken hearth, central to the Japanese home. Families gathered around it for warmth, cooking, and stories. It's the heart of the home—just like this project aspires to be the heart of your family's digital memories.

## License

Dual licensed under MIT OR Apache-2.0
