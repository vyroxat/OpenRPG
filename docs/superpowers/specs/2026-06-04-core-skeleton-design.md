# Core Skeleton Design

## Purpose

Build the first Rust-first OpenRPG implementation slice: a small, deterministic `openrpg-core` crate that can act as the nucleus for later RPG systems.

## Scope

This slice covers Milestone 1 from `openrpg-spec.md`:

- Cargo workspace structure.
- `openrpg-core` crate.
- Engine creation and configuration.
- Module registration and dependency validation.
- Entity registry.
- Component schema registration.
- Event bus.
- Interceptor pipeline.
- Command execution API.
- Tick model.
- Seeded RNG.
- State patches.
- Validation foundation.

It intentionally excludes content file loading, inventory, equipment, combat, quests, saves, tooling CLI, and bindings.

## Architecture

The repository becomes a Cargo workspace with `crates/openrpg-core` as the first member. The crate exposes `OpenRpgCore`, a headless engine that owns registries, runtime state, command handlers, interceptors, events, patches, and deterministic RNG state.

The engine API stays deliberately plain and serializable. Commands accept typed Rust structs at the boundary for core tests, while payload and component data use `serde_json::Value` for the early schema-neutral foundation. Later modules can wrap these values in richer typed APIs.

## Components

- `engine`: owns runtime state, module registration, commands, events, patches, ticks, and RNG.
- `entity`: stores entity IDs, tags, metadata, and component data.
- `component`: registers component schemas by namespaced ID.
- `command`: defines command requests, handlers, results, and errors.
- `event`: defines committed event records and an event listener queue.
- `interceptor`: allows ordered pre-resolution command interception.
- `patch`: records state changes for sync, debugging, and tests.
- `rng`: wraps seeded deterministic random generation.
- `validation`: provides reusable validation errors and helper checks.

## Data Flow

Callers create an engine, register modules/components/commands/interceptors, then call `boot()`. After boot, callers execute commands. A command may create entities, update components, emit events, and record state patches. `tick(delta_ms)` advances the logical tick and returns accumulated events and patches since the last drain.

## Error Handling

Core errors use stable machine-readable codes plus messages and optional JSON details. Boot fails on missing module dependencies or duplicate registrations. Commands fail when unknown, denied by an interceptor, or rejected by handler validation.

## Testing

Implementation uses TDD. Integration tests define the public API first:

- Engine boots registered modules in dependency order.
- Missing module dependencies fail boot.
- Entity creation records a patch and can be queried.
- Registered commands emit events and patches.
- Interceptors can deny commands before handlers run.
- Ticks advance deterministically and drain event/patch output.
- Seeded RNG produces stable sequences.

## Acceptance Criteria

- `cargo test --workspace` passes.
- `openrpg-core` compiles without warnings.
- Public types are documented enough for early users to orient.
- The crate remains headless and has no renderer, input, audio, or platform assumptions.
