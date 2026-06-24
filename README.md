# OpenRPG

A modular, extensible backend framework for RPG mechanics.

See `openrpg-spec.md` for full specification.

## Current Status

Basic core architecture implemented in Rust following the spec (entities, engine, commands, events, etc.). More modules coming soon.

## Baseplate Debug World

OpenRPG keeps a baseplate debug world for tested backend/frontend integration snippets. See `docs/baseplate/openrpg-core-debug-world.md` and run:

```bash
cargo run -p openrpg-core --example baseplate_debug_world
```
