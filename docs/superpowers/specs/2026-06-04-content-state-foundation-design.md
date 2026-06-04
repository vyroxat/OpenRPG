# Content and State Foundation Design

## Purpose

Extend `openrpg-core` beyond the skeleton with the first Milestone 2 foundations: namespaced content IDs, an in-memory content registry, and serializable state snapshots.

## Scope

This slice covers:

- Namespaced ID parsing and validation.
- Duplicate content ID detection.
- Content entries stored by stable final ID.
- Read-only content lookup by ID and kind.
- Engine-level world value mutation outside command handlers.
- State snapshots for tick, world values, content registry, and entity registry.
- JSON round-tripping of state snapshots.
- Snapshot restore with schema version validation.

It intentionally leaves file loading for TOML/YAML/JSON, migrations, and standalone validation CLI for later slices.

## Architecture

`NamespacedId` is a small validated value object that enforces `namespace:value`. The content registry accepts `ContentEntry` values and validates their IDs before storage. It exposes a read-only view for ID and kind queries while keeping mutation behind `content_mut()`. State snapshots live in `engine` because they are cross-cutting: they serialize the runtime state owned by the engine without exposing the internal registries as mutable implementation details.

## Testing

Integration tests cover valid and invalid namespaced IDs, duplicate content IDs, invalid content IDs, content lookup by ID and kind, snapshot/restore of entities, world values, and tick state, and JSON snapshot round-tripping.
