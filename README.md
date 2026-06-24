# OpenRPG

A modular, extensible backend framework for RPG mechanics.

See `openrpg-spec.md` for full specification.

## Current Status

OpenRPG currently has a Rust core with module boot, JSON content packs, entity/component storage, command execution, events, patches, deterministic RNG, world values, snapshots, stat blocks, resource pools, and stackable inventories. More RPG systems are coming next.

## Baseplate Debug World

OpenRPG keeps a baseplate debug world for tested backend/frontend integration snippets. See `docs/baseplate/openrpg-core-debug-world.md` and run:

```bash
cargo run -p openrpg-core --example baseplate_debug_world
```
