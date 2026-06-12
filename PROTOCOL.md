# Hearth Sync Protocol

The Hearth sync protocol enables efficient multi-device synchronization of shared collections. It's designed for:

- **Offline support** — Sync only when connected
- **Bandwidth efficiency** — Delta-based changes, not full state
- **Conflict resolution** — Client and server can detect and resolve conflicts
- **Extensibility** — Support for multiple transport implementations

## Overview

The protocol is stateless request-response. Clients maintain a **cursor** from the previous sync to request only new changes.

```
Client                          Server
  |                              |
  +------ Identify Request ------>|
  |<----- Client State -----------+
  |                              |
  +--- Fetch Changes (cursor) --->|
  |<----- ChangeSet (cursor) -----+
  |                              |
  +--- Push Changes ------------>|
  |<----- SyncAck (conflicts) ----+
  |                              |
  +--- Upload File ----------->|
  |<--- FileHandle ----------+
```

## Endpoints

All endpoints require authentication via `Authorization: Bearer <token>` header.

### 1. Identify

**Request:**
```
POST /sync/identify
Content-Type: application/json

{
  "client_id": "client-uuid-or-device-id",
  "version": "0.1.0",
  "last_sync": "optional-cursor-from-last-sync"
}
```

**Response:**
```json
{
  "client_id": "client-uuid-or-device-id",
  "version": "0.1.0",
  "last_sync": "cursor-for-next-fetch"
}
```

### 2. Fetch Changes

Get changes since the last sync cursor.

**Request:**
```
POST /sync/fetch
Content-Type: application/json

{
  "cursor": "optional-cursor-from-identify",
  "limit": 100
}
```

**Response:**
```json
{
  "cursor": "next-cursor-to-use",
  "changes": [
    {
      "id": "change-uuid",
      "change_type": "created",  // "created", "updated", "deleted"
      "resource_id": "photo-uuid",
      "timestamp": "2024-06-12T10:30:00Z",
      "data": {
        "name": "summer-2024.jpg",
        "resource_type": "photo",
        "size_bytes": 2048576,
        "mime_type": "image/jpeg"
      }
    }
  ],
  "has_more": false,
  "checkpoint": "optional-checkpoint-for-progress"
}
```

### 3. Push Changes

Send local changes to the server.

**Request:**
```
POST /sync/push
Content-Type: application/json

{
  "changes": [
    {
      "id": "local-change-uuid",
      "change_type": "created",
      "resource_id": "photo-uuid",
      "data": {
        "name": "vacation.jpg",
        "resource_type": "photo",
        "collection_id": "collection-uuid"
      }
    }
  ]
}
```

**Response:**
```json
{
  "received_count": 1,
  "cursor": "updated-cursor",
  "conflicts": [
    {
      "resource_id": "photo-uuid",
      "client_version": "version-1",
      "server_version": "version-2"
    }
  ]
}
```

### 4. Upload File

Upload a resource file (photo, video, document).

**Request:**
```
POST /sync/upload
Content-Type: multipart/form-data

name: "summer-2024.jpg"
mime_type: "image/jpeg"
resource_type: "photo"
file: <binary data>
```

**Response:**
```json
{
  "id": "resource-uuid",
  "path": "resources/user-id/resource-uuid.jpg",
  "size": 2048576,
  "checksum": "sha256:abc123..."
}
```

### 5. Download File

Download a resource file.

**Request:**
```
GET /sync/download/{resource_id}
```

**Response:**
```
Binary file content
```

## Change Types

| Type | Meaning | Data |
|------|---------|------|
| `created` | New resource or collection | Full resource data |
| `updated` | Modified resource | Changed fields |
| `deleted` | Removed resource | Resource ID only |

## Cursors

A cursor is an opaque string that represents a point in time on the server. It's used to fetch only changes since that point.

- **First sync** — Omit cursor or pass `null` to get all changes
- **Subsequent syncs** — Use the cursor from the previous response
- **Cursor validity** — Cursors are valid for 30 days (configurable)

## Conflict Resolution

When a client and server have diverging edits to the same resource, the server returns conflict info in the `SyncAck`:

```json
{
  "conflicts": [
    {
      "resource_id": "photo-uuid",
      "client_version": "v1-local-edit",
      "server_version": "v2-from-other-device"
    }
  ]
}
```

**Resolution strategies:**
1. **Last write wins** — Client re-fetches the resource and overwrites with server version
2. **Manual merge** — Client presents both versions to user
3. **Three-way merge** — Client has base, local, and remote version (future)

## Checksum Validation

On file upload, the server computes a SHA-256 checksum and returns it. The client should verify:

```
client_checksum = sha256(file_data)
server_checksum = response.checksum
assert client_checksum == server_checksum
```

## Bandwidth Optimization

### Delta Sync

Instead of uploading the full file on update, the protocol supports delta patches (future):

```json
{
  "change_type": "updated",
  "resource_id": "photo-uuid",
  "delta": {
    "patch": "binary-delta-bytes",
    "patch_type": "xdelta3"
  }
}
```

### Compression

Large uploads should be compressed client-side:

```
POST /sync/upload
Content-Encoding: gzip
...
```

## Error Handling

All errors return a standard error response:

```json
{
  "error": "invalid_cursor",
  "details": "Cursor expired (30 days old)"
}
```

**HTTP status codes:**
- `400` — Invalid request
- `401` — Unauthorized
- `403` — Forbidden (no access to resource)
- `404` — Not found
- `409` — Conflict (client version mismatch)
- `413` — Payload too large
- `500` — Server error

## Protocol Versions

The protocol is versioned via the `version` field in the `Identify` request. The server responds with its version, and the client should handle version mismatches gracefully.

Current version: `0.1.0`

## Future Extensions

- **Delta patches** — Efficient large file updates
- **Bandwidth-aware batching** — Adjust batch sizes based on connection speed
- **P2P sync** — Direct device-to-device sync when on same network
- **Snapshot restore** — Rollback to previous state
