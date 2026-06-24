# OpenRPG Core Baseplate Debug World

The baseplate is OpenRPG's backend debug world. It is a growing collection of compile-checked snippets and integration tests that show how current engine pieces work together.

Use it for three jobs:

- Verify the current backend surface works as a system.
- Provide snippets for wiki and documentation pages.
- Give frontend or tooling code a stable example of command, event, patch, query, and save/restore flow.

## Verified Surface

Current baseplate coverage:

- Engine creation and module boot.
- JSON content-pack loading.
- Entity creation with component data.
- Stat and resource components attached to an entity.
- Flat and percent stat modifier resolution.
- Resource current/max clamping and serialization.
- Inventory stack data attached to an entity.
- Content-backed inventory stack limits.
- Equipment slot data attached to an entity.
- Equipment stat modifiers applied to the hero stat block.
- Frontend-to-backend command execution.
- Backend-to-frontend events and patches through `tick`.
- Deterministic RNG.
- Snapshot JSON encode/decode and restore.

## Source Of Truth

The executable snippet lives in:

```text
crates/openrpg-core/examples/baseplate_debug_world.rs
```

The integration suite lives in:

```text
crates/openrpg-core/tests/baseplate.rs
```

When adding a backend feature, add it to the baseplate once it has a stable public usage pattern. Documentation snippets should come from the same pattern so examples do not drift away from tested behavior.

## Run It

```bash
cargo run -p openrpg-core --example baseplate_debug_world
```

Expected behavior: the example boots the engine, loads content, creates a hero, attaches stats/resources, sends a command, advances a tick, and prints frontend-facing event/patch counts plus queried content and hero mechanics values.

Current example output:

```text
tick: 1
events for frontend: 1
patches for frontend: 5
hero: entity:hero
hero strength: 15
hero health: 80
hero potions: 3
hero main hand: mygame:iron_sword
potion max stack: 99
```
