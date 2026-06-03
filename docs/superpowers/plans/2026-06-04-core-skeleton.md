# Core Skeleton Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first Rust `openrpg-core` crate with a tested engine nucleus.

**Architecture:** Create a Cargo workspace and keep the core crate split into focused modules for engine state, commands, entities, modules, events, interceptors, patches, RNG, and validation. Integration tests drive the public API before implementation.

**Tech Stack:** Rust 2024 edition, Cargo workspace, `serde`, `serde_json`, `rand`, `rand_chacha`, `thiserror`.

---

### Task 1: Workspace and Core API Tests

**Files:**
- Create: `Cargo.toml`
- Create: `crates/openrpg-core/Cargo.toml`
- Create: `crates/openrpg-core/src/lib.rs`
- Create: `crates/openrpg-core/tests/core_flow.rs`

- [ ] **Step 1: Write failing integration tests**

Create tests for module boot, entity creation, command execution, interceptors, tick draining, and seeded RNG.

- [ ] **Step 2: Run tests to verify red**

Run: `cargo test --workspace`

Expected: FAIL because the public API does not exist yet.

- [ ] **Step 3: Implement minimal module files**

Create focused files under `crates/openrpg-core/src/` and export public types from `lib.rs`.

- [ ] **Step 4: Run tests to verify green**

Run: `cargo test --workspace`

Expected: PASS.

### Task 2: Format and Verify

**Files:**
- Modify: Rust source files from Task 1

- [ ] **Step 1: Format**

Run: `cargo fmt --all`

- [ ] **Step 2: Test**

Run: `cargo test --workspace`

Expected: PASS.

- [ ] **Step 3: Check**

Run: `cargo check --workspace`

Expected: PASS.
