# Content and State Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Milestone 2 foundations for namespaced content IDs, content registration, and state snapshots.

**Architecture:** Keep content ID validation separate from the registry. Keep snapshot creation/restoration on `OpenRpgCore` because it owns the runtime state being captured.

**Tech Stack:** Rust 2024 edition, `serde`, `serde_json`, existing `openrpg-core`.

---

### Task 1: Namespaced IDs

**Files:**
- Create: `crates/openrpg-core/src/namespaced_id.rs`
- Modify: `crates/openrpg-core/src/lib.rs`
- Test: `crates/openrpg-core/tests/content_state.rs`

- [x] **Step 1: Write failing tests for valid and invalid namespaced IDs**
- [x] **Step 2: Run `cargo test --workspace` and confirm missing API failure**
- [x] **Step 3: Implement `NamespacedId::parse` with stable error codes**
- [x] **Step 4: Run `cargo test --workspace` and confirm passing tests**

### Task 2: Content Registry

**Files:**
- Create: `crates/openrpg-core/src/content.rs`
- Modify: `crates/openrpg-core/src/engine.rs`
- Modify: `crates/openrpg-core/src/lib.rs`
- Test: `crates/openrpg-core/tests/content_state.rs`

- [x] **Step 1: Write failing tests for duplicate and invalid content IDs**
- [x] **Step 2: Run `cargo test --workspace` and confirm missing API failure**
- [x] **Step 3: Implement `ContentEntry`, `ContentRegistry`, and `OpenRpgCore::content_mut`**
- [x] **Step 4: Run `cargo test --workspace` and confirm passing tests**

### Task 3: State Snapshots

**Files:**
- Modify: `crates/openrpg-core/src/engine.rs`
- Modify: `crates/openrpg-core/src/entity.rs`
- Test: `crates/openrpg-core/tests/content_state.rs`

- [x] **Step 1: Write failing test for snapshot/restore of tick, world, and entities**
- [x] **Step 2: Run `cargo test --workspace` and confirm missing API failure**
- [x] **Step 3: Implement `StateSnapshot`, `snapshot`, `restore_snapshot`, and `current_tick`**
- [x] **Step 4: Run `cargo test --workspace` and confirm passing tests**

### Task 4: Read APIs and JSON Round-Trip

**Files:**
- Modify: `crates/openrpg-core/src/content.rs`
- Modify: `crates/openrpg-core/src/engine.rs`
- Test: `crates/openrpg-core/tests/content_state.rs`

- [x] **Step 1: Write failing tests for read-only content queries and JSON snapshot round-trip**
- [x] **Step 2: Run `cargo test --workspace` and confirm missing API failure**
- [x] **Step 3: Implement `ContentRegistryRef`, `OpenRpgCore::content`, `get`, and `ids_for_kind`**
- [x] **Step 4: Run `cargo test --workspace` and confirm passing tests**
