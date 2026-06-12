# Irori (囲炉裏) Development Guide

**Stack:** Rust, Axum, PostgreSQL, Protocol-based sync

> *Like the traditional Japanese irori—a sunken hearth at the center of the home—our architecture has a warm center where memories and logic converge, with clear paths for different transport layers to approach it.*

## Backlog & Followup Work

When you identify a meaningful improvement, bug, or refactor that is **out of scope for the current task**, surface it to the user and **ask whether to file a GitHub issue**. Do not:

- File issues unilaterally — issue spam pollutes the tracker; the user decides what's worth tracking
- Bury it in a `TODO` comment that nobody re-reads
- Bolt it onto the current PR as "while I'm here" scope creep
- Leave it unmentioned

**Pattern:** "I noticed [thing]. This matters because [why]. Want me to file an issue?"

**Exception:** A `TODO:` comment is acceptable when it points at a very specific tactical follow-up tightly coupled to the surrounding code (e.g., "TODO: move to service layer when MCP or another transport consumes this"). It is NOT acceptable for open-ended ideas or anything requiring more than ~10 lines to act on.

## Architecture

Irori separates **domain logic** (business rules) from **transport layers** (how clients communicate):

- **`services/`** — Transport-agnostic business logic. No knowledge of HTTP, MCP, or sync protocol. Each domain (resources, collections, sharing, users) has trait definitions.
- **`api/`** — HTTP transport layer. Thin wrappers that extract HTTP params, call service methods, return JSON responses.
- **`mcp/`** — MCP protocol transport. Same pattern: thin wrappers around service methods.
- **`app.rs`** — `AppState` struct, shared across all transports (the irori's flame).
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
- `IroriSyncProtocol` — Default efficient protocol
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

## Models Separation (Architectural Rule)

**All public data types live in `models.rs` files, never inline in implementation files.**

This is **strict**—no exceptions, no judgment calls. The reason: it keeps data types organized, prevents drift, and makes the architecture clear.

**Pattern:**
```
services/foo/
├── mod.rs          # Trait definitions and implementations only
└── models.rs       # All pub structs, enums, request/response types
```

**In `models.rs`:**
- Data transfer objects (DTOs): `CreateFooRequest`, `UpdateFooRequest`
- Domain models: `Foo`, `FooDetails`
- Enums: `FooStatus`, `FooType`
- Any other `pub struct` or `pub enum`

**In `mod.rs`:**
- Trait definitions: `pub trait FooService { ... }`
- Re-exports: `pub use models::{Foo, CreateFooRequest};`
- Helper functions (no `pub` data types)

**Why this matters:**
- Future developers see `pub use models::*` and know where to look
- Data types can't drift—single definition, single place
- Easier to find what affects API/MCP transport (look in any `models.rs`)

**Enforced by convention.** If someone adds a `pub struct` directly in `mod.rs`, call it out in review and request they move it to `models.rs`.

## Adding a New Domain

1. **Create `services/<name>/models.rs`** with all data types:
   ```rust
   #[derive(Clone, Debug, Serialize, Deserialize)]
   pub struct Foo {
       pub id: Uuid,
       // ...
   }
   
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct CreateFooRequest {
       // ...
   }
   ```

2. **Create `services/<name>/mod.rs`** with the trait and re-exports:
   ```rust
   pub mod models;
   use async_trait::async_trait;
   pub use models::{Foo, CreateFooRequest};
   
   #[async_trait]
   pub trait FooService: Send + Sync {
       async fn create(...) -> Result<Foo>;
       async fn get(...) -> Result<Option<Foo>>;
   }
   ```

3. **Create a repository** in `services/repos/foo_repo.rs` (implements trait) — future phase

4. **Add HTTP routes** in `api/foo/mod.rs` with `pub fn router() -> Router<Arc<AppState>>`

5. **Add MCP tools** in `mcp/foo.rs` (thin wrappers around the same service)

6. **Register in** `app.rs` (add to `AppState`)

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

## Style Guidelines

- **Iterators over loops** — Use `filter_map`, `map`, `collect` instead of imperative loops
- **Early returns** — Use `?` operator, `let-else` for pattern matching with early exit
- **Flatten** — Use `.ok()`, `.and_then()`, and chains over nested matches
- **No `#[allow(...)]` suppressions** — Fix the root cause instead:
  - Unused imports → remove them
  - Too many arguments → use a struct or builder pattern
  - Dead code → delete it (mark as `#[allow(dead_code)]` only for intentional stubs)
  - Redundant closures → pass the function directly
- **Tracing** — Add `#[tracing::instrument]` to all route handlers (skip large args like state, body)
- **File ordering** (stepdown rule) — Source files read top-down:
  1. `use` declarations
  2. Public API (handlers, service methods)
  3. Private helpers at the bottom in a `// ── Helpers ──` section
  4. Within public API: caller appears above callee where possible (descend one abstraction level per function)
  5. Exception: a helper used by exactly one handler can sit immediately below it for locality

Example layout for `api/resources/routes.rs`:
```rust
use ...;

pub async fn upload_resource(...) { ... }  // route handlers first
pub async fn list_resources(...) { ... }

// ── Helpers ──

async fn validate_upload(...) { ... }
fn extract_mime_type(...) { ... }
```

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
docker-compose logs -f irori
```

The Dockerfile uses multi-stage build (builder → runtime) to keep the final image small.

## Migrations

Database migrations live in `src/main.rs` as CLI commands and in `src/core/db.rs` as schema initialization:

```bash
cargo run -- migrate --list              # Show available migrations
cargo run -- migrate --name m001_init    # Dry run (default, no writes)
cargo run -- migrate --name m001_init --apply  # Execute
```

**Rules:**
- Migrations must accept a `dry_run: bool` parameter and perform **zero writes** when true
- Log what *would* happen in dry-run mode (e.g., "Would add 50 users to collection X")
- Migrations should be idempotent — skip records that are already migrated
- Each migration is a separate function: `m001_init_schema`, `m002_add_columns`, etc.

As the schema grows, consider migrating to `sqlx::migrate!` with `.sql` files, but Rust migrations offer flexibility.

## Testing

Tests must live in **separate files**, not inline in source files:

- **No inline `#[cfg(test)] mod tests { ... }` blocks** — only acceptable form is a one-line module declaration
- **Unit tests** — Put in `<stem>_tests.rs` next to the source file, and reference from source:

  ```rust
  #[cfg(test)]
  #[path = "foo_tests.rs"]
  mod tests;
  ```

- **Integration tests** — Go in `server/tests/` and `cli/tests/` directories
- Use `_tests.rs` suffix (not `tests.rs`) to avoid collisions

Run tests before committing:
```bash
cargo test
RUST_BACKTRACE=1 cargo test  # With backtrace on failure
```

## Git Workflow

```bash
# Create feature branch
git checkout -b feat/sync-protocol

# Make changes, format, lint, test
cargo fmt
cargo clippy -- -D warnings
cargo test

# Commit with meaningful message
git add .
git commit -m "Add sync protocol abstraction

Implement trait-based sync protocol to allow pluggable protocol implementations."

# Push and create PR
git push origin feat/sync-protocol
```

Meaningful commit messages: focus on the **why**, not the what (code shows the what).

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

## Cargo Features

- `api` — HTTP API routes (enabled by default)
- `mcp` — MCP protocol server (enabled by default)
- Both can be independently disabled: `cargo build --no-default-features --features mcp`
- **CI runs with default features (both enabled).**

This allows optional compilation of heavy dependencies (like MCP libraries) when you only need the API.

## CI/CD

GitHub Actions runs on every push to `main` and on pull requests:

- **server-ci.yml** — Runs format check, clippy lints, and tests for the server (always)
- **cli-ci.yml** — Runs format check, clippy lints, and tests for the CLI (only when CLI code changes)

Both jobs run in parallel with Rust caching to speed up builds. **All checks must pass before merging to main.**

### Before Pushing

Always run these three checks locally:

```bash
cargo fmt -- --check      # Formatting (must match)
cargo clippy -- -D warnings  # No warnings allowed
cargo test                 # All tests pass
```

If `cargo fmt` says files need formatting, run it without `--check` to fix them:
```bash
cargo fmt
```

### CLI Tips

- `cargo check` — Fast syntax check (no codegen)
- `RUST_BACKTRACE=1 cargo test` — Full backtrace on test failure
- `cargo test -- --nocapture` — Show println! output during tests
- `cargo clippy --all-targets` — Run on tests and examples too

## Questions?

For architectural decisions, implementation patterns, or anything unclear, refer back to this document. It's a living guide—update it as patterns emerge.
