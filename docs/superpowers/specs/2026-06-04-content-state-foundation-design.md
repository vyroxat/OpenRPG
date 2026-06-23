# Content and State Foundation Design

## Purpose

Extend `openrpg-core` beyond the skeleton with the first Milestone 2 foundations: namespaced content IDs, an in-memory content registry, and serializable state snapshots.

## Scope

This slice covers:

- Namespaced ID parsing and validation.
- Duplicate content ID detection.
- Content entries stored by stable final ID.
- Read-only content lookup by ID and kind.
- JSON content-pack parsing into registry entries.
- Atomic content-pack loading so failed validation does not partially mutate state.
- Engine convenience loading from JSON content-pack strings.
- Engine-level world value mutation outside command handlers.
- State snapshots for tick, world values, content registry, and entity registry.
- JSON round-tripping of state snapshots.
- Snapshot restore with schema version validation.

It intentionally leaves file loading for TOML/YAML/JSON, migrations, and standalone validation CLI for later slices.

## Architecture

`NamespacedId` is a small validated value object that enforces `namespace:value`. The content registry accepts `ContentEntry` values and validates their IDs before storage. It exposes a read-only view for ID and kind queries while keeping mutation behind `content_mut()`. `ContentPack` parses JSON into a batch of entries, and pack loading clones the registry before validation so an error leaves previous content untouched. `OpenRpgCore::load_content_pack_json` is a convenience edge API that routes through the same parsing and atomic load path. State snapshots live in `engine` because they are cross-cutting: they serialize the runtime state owned by the engine without exposing the internal registries as mutable implementation details.

## Testing

Integration tests cover valid and invalid namespaced IDs, duplicate content IDs, invalid content IDs, content lookup by ID and kind, JSON content-pack parsing, direct engine JSON loading, atomic failed pack loads, snapshot/restore of entities, world values, and tick state, and JSON snapshot round-tripping.

The project also keeps a baseplate verification suite in `crates/openrpg-core/tests/baseplate.rs`. Baseplate tests should grow as engine capabilities grow and should prove that independently added pieces still work together in a representative flow.
