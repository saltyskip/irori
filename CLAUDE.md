# Hearth Development Guide

**Stack:** Rust, Axum, PostgreSQL, Protocol-based sync

## Architecture

Hearth separates **domain logic** (business rules) from **transport layers** (how clients communicate):

- **`services/`** — Transport-agnostic business logic. No knowledge of HTTP, MCP, or sync protocol. Each domain (resources, collections, sharing, users) has trait definitions.
- **`api/`** — HTTP transport layer. Thin wrappers that extract HTTP params, call service methods, return JSON responses.
- **`mcp/`** — MCP protocol transport. Same pattern: thin wrappers around service methods.
- **`app.rs`** — `AppState` struct, shared across all transports.
- **`core/`** — Infrastructure only: database connections, storage backends, config. No business logic.

**Key rule: Transport layers must not import from each other.** Both `api/` and `mcp/` import from `services/`, but never from each other.

### Service Trait Pattern

Every domain has trait definitions in `services/<domain>.rs`:

```rust
#[async_trait]
pub trait ResourceService: Send + Sync {
    async fn upload(...) -> Result<Resource>;
    async fn get(...) -> Result<Option<Resource>>;
    // ...
}
```

**Implementations can be swapped:**
- Concrete implementations in repo crates (e.g., `services/repos/resources.rs`)
- Mock implementations for testing
- Alternative strategies for different use cases

### Sync Protocol Pluggability

The sync protocol is defined as a trait in `services/sync.rs`:

```rust
#[async_trait]
pub trait SyncProtocol: Send + Sync {
    async fn identify(...) -> Result<ClientState>;
    async fn fetch_changes(...) -> Result<ChangeSet>;
    async fn push_changes(...) -> Result<SyncAck>;
    // ...
}
```

Implementations:
- `HearthSyncProtocol` — Default efficient protocol
- `ImmichSyncProtocol` — Compatibility layer for Immich clients

Both are wired at startup via config.

### Storage Backend Pluggability

`core/storage.rs` defines the storage abstraction:

```rust
#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn upload(&self, path: &str, data: &[u8]) -> Result<()>;
    async fn download(&self, path: &str) -> Result<Vec<u8>>;
    // ...
}
```

Implementations:
- `LocalStorage` — Filesystem (development)
- `NasStorage` — NFS mount (your UNASPro)
- `S3Storage` — AWS S3 (future)

Selected at startup via `STORAGE_BACKEND` config.

## Adding a New Domain

1. **Define the service trait** in `services/<name>.rs`:
   ```rust
   #[async_trait]
   pub trait FooService: Send + Sync {
       async fn create(...) -> Result<Foo>;
       async fn get(...) -> Result<Option<Foo>>;
   }
   ```

2. **Create a repository** in `services/repos/foo_repo.rs` (implements trait)

3. **Add HTTP routes** in `api/foo/mod.rs` with `pub fn router() -> Router<Arc<AppState>>`

4. **Add MCP tools** in `mcp/foo.rs` (thin wrappers around the same service)

5. **Register in** `app.rs` (add to `AppState`)

## Migrations

Database migrations are defined in `src/main.rs` and applied via CLI:

```bash
cargo run -- migrate --name m001_init_schema --apply
```

For now, migrations are Rust functions in `src/migrations/`. As the schema grows, consider:
- Using `sqlx::migrate!` macro with `.sql` files
- Or keeping Rust migrations for maximum flexibility

## Testing

Tests live in **separate files**, not inline:

- **Unit tests** — `<stem>_tests.rs` next to source files
- **Integration tests** — `tests/` directory

```rust
// In src/api/health_tests.rs
#[tokio::test]
async fn test_health_endpoint() {
    // ...
}
```

Run with: `cargo test`

## Style

- **Iterators over loops** — Use `filter_map`, `map`, `collect`
- **Early returns** — Use `?` operator and `let-else`
- **Type safety** — Prefer `Result<T>` over `Option<bool>`
- **Async/await** — Use Tokio runtime, `#[tokio::main]`
- **No `panic!` in libraries** — Return `Err` instead
- **Logging** — Use `tracing::info!`, `tracing::error!`

## Configuration

All runtime config comes from environment variables (see `core/config.rs`):

```bash
DATABASE_URL
STORAGE_BACKEND
STORAGE_PATH
NAS_MOUNT_PATH
SYNC_PROTOCOL
JWT_SECRET
ENVIRONMENT
```

No hardcoded paths or secrets.

## Deployment

Docker setup is in `server/Dockerfile` and `docker-compose.yml`:

```bash
# Using OrbStack
orbstack start
docker-compose up

# View logs
docker-compose logs -f hearth
```

The Dockerfile uses multi-stage build (builder → runtime) to keep the final image small.

## Git Workflow

```bash
# Create feature branch
git checkout -b feat/sync-protocol

# Make changes, test
cargo test

# Commit
git add .
git commit -m "Add sync protocol abstraction"

# Push and create PR (later)
git push origin feat/sync-protocol
```

Meaningful commit messages with context (not just "fix bug").

## What's Next

See `README.md` for the current roadmap. High-priority items:

1. ✅ Service trait definitions (done)
2. ✅ Storage abstraction (done)
3. ✅ Basic API scaffold (done)
4. ⏳ Implement resource upload/download
5. ⏳ Implement sync protocol
6. ⏳ Implement collections and sharing
7. ⏳ Add MCP tools for Claude integration
8. ⏳ User authentication and JWT tokens

## Questions?

For architectural decisions, implementation patterns, or anything unclear, refer back to this document. It's a living guide—update it as patterns emerge.
