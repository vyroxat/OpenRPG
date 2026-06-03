# OpenRPG Full Specification

**Version:** 0.3.0-draft
**Status:** Pre-RFC
**License:** MIT (proposed)

## 1. Executive Summary

OpenRPG is a headless, modular RPG mechanics engine. It provides the underlying systems that well-built RPGs usually need: entities, stats, resources, inventory, equipment, abilities, effects, combat, progression, quests, dialogue, world state, economy, factions, encounters, loot, saves, validation, simulation, and extension hooks.

OpenRPG is not a rendered game and not a full game engine. It does not own graphics, controls, audio, camera, physics, input, platform packaging, or the final player experience. It is the mechanical substrate beneath an RPG. Developers connect their frontend, renderer, game engine, UI, narrative layer, or server to OpenRPG and let OpenRPG handle the reusable RPG logic.

The core promise:

> Developers define the rules, content, and frontend. OpenRPG runs the mechanical systems consistently.

OpenRPG should let developers build RPGs the way strong web and systems ecosystems work: a small mandatory core, many opt-in packages, clear contracts, deterministic state transitions, and extension points that prevent teams from needing to fork the engine.

## 2. Product Vision

OpenRPG should become the reliable mechanical heart of RPG projects. A developer should be able to start from content definitions and frontend code, then hook into OpenRPG for common RPG behavior instead of rebuilding core systems from scratch.

OpenRPG should support:

- A small fantasy JRPG with menus and turn-based combat.
- A tactical RPG with grid positioning and initiative.
- A narrative RPG with dialogue, quests, faction reputation, and skill checks.
- A roguelite with procedural encounters, loot tables, progression, and deterministic seeds.
- A survival RPG with hunger, stamina, crafting extensions, world flags, and status effects.
- A multiplayer RPG server that runs authoritative mechanics without rendering.

The engine must be broad enough to support different RPG styles, but not so opinionated that it becomes a narrow template.

## 3. Goals

- Provide reusable RPG systems that can be integrated into different frontends and game engines.
- Keep presentation strictly separate from mechanics.
- Ship as a modular monorepo with a mandatory core and opt-in packages.
- Make every major system data-driven and schema-validated.
- Make runtime state serializable, inspectable, and deterministic.
- Support seeded randomness for tests, saves, replays, rollback, and netcode.
- Expose a clear command, event, patch, and trace API.
- Let developers replace rules through hooks, interceptors, resolvers, plugins, and custom schemas.
- Provide tooling for validation, debugging, simulation, balancing, and migration.
- Support modding through namespaced content packs, dependency ordering, and conflict detection.
- Be implementation-neutral at the specification level while recommending an initial reference direction.

## 4. Non-Goals

OpenRPG explicitly does not provide:

- Rendering.
- Asset loading.
- Animation.
- Audio playback.
- Input handling.
- Camera behavior.
- Physics or collision detection.
- Platform packaging.
- Cutscene playback.
- A built-in map editor for the MVP.
- A required networking transport.
- A required frontend framework.
- A required game engine.

Planned future packages may help with world generation(openrpg-world) or networking(openrpg-networking), but OpenRPG remains a mechanics engine, not a complete game engine.

## 5. Target Users

### 5.1 Game Developers

Developers building RPGs who want a dependable mechanics layer instead of writing inventory, combat, quests, progression, and save systems from scratch.

### 5.2 Technical Designers

Designers who want to author stats, abilities, items, quests, dialogue, encounters, and progression in structured data files.

### 5.3 Tool Builders

Developers building editors, modding tools, simulation tools, balance reports, content pipelines, or game-specific design tooling.

### 5.4 Server Developers

Developers who want to run RPG mechanics in headless server environments for authoritative multiplayer or persistent worlds.

### 5.5 Mod Authors

Community creators who add new content packs, items, quests, enemies, abilities, status effects, or rules extensions.

## 6. Core Design Principles

### 6.1 Headless by Default

OpenRPG exposes mechanics and state. The consuming game decides how to visualize them.

Example:

- OpenRPG resolves that an attack hit, dealt 12 fire damage, applied burning for 3 turns, and consumed 8 mana.
- The frontend decides how that looks, sounds, animates, and feels.

### 6.2 Hookable, Not Over-Opinionated

OpenRPG should provide practical defaults, but developers must be able to intercept, override, or replace behavior at the right boundaries.

### 6.3 Modular and Composable

The core package is mandatory. Everything else is opt-in. A game can use inventory and quests without combat, or combat and progression without dialogue.

### 6.4 Serializable First

All state must be serializable at any point. Save/load, debugging, replay, migration, and server sync are core design concerns.

### 6.5 Deterministic Mechanics

Given the same content, state, seed, and command sequence, OpenRPG should produce the same result. This enables reproducible tests, combat simulations, replays, rollback, and authoritative server verification.

### 6.6 Data-Driven Content

Most game-specific behavior should come from validated content definitions rather than engine source changes.

### 6.7 Explicit State

Hidden state should be avoided. Engine state should be inspectable, queryable, serializable, diffable, and testable.

### 6.8 Extension Without Forking

Developers should add mechanics through plugins, hooks, resolvers, content schemas, and middleware instead of editing OpenRPG internals.

### 6.9 Frontend Freedom

OpenRPG must not assume one renderer, UI paradigm, camera style, input model, or game engine. A browser game, Godot game, Unity game, terminal prototype, custom engine, or server should all be able to consume the same mechanical concepts.

## 7. Reference Implementation Direction

The canonical specification is language-agnostic.

Recommended reference direction:

- **Core implementation:** Rust, for performance, determinism, memory safety, and server suitability.
- **Official bindings:** TypeScript/JavaScript, Python, and GDScript.
- **Package model:** monorepo with independently versioned crates/packages.
- **Data formats:** TOML, YAML, and JSON accepted at the edges, unified into one internal schema model.
- **Formula language:** a safe expression language first; optional scripting package later.

This recommendation is not a hard requirement for alternate implementations. The contract is the command model, state model, event model, deterministic behavior, and content schema behavior.

## 8. Monorepo Package Architecture

OpenRPG should be organized like a modular ecosystem: similar in spirit to Tokio features in Rust or Babel packages in JavaScript. `openrpg-core` is mandatory. All other packages are opt-in.

```text
openrpg-core
  mandatory nucleus:
  entity registry, component schema, event bus, interceptor pipeline,
  command pipeline, tick engine, content registry, config loader,
  deterministic RNG, state patches, save system, validation foundation

openrpg-combat
  depends on: core
  turn-based, ATB, action-resolution, initiative, targeting, damage,
  combat sessions, combat traces

openrpg-progression
  depends on: core
  XP, levels, skill trees, perks, class rules, milestones

openrpg-inventory
  depends on: core
  item instances, stacks, containers, transfers, weight, volume

openrpg-equipment
  depends on: core, inventory
  equipment slots, requirements, stat modifiers, durability, set bonuses

openrpg-abilities
  depends on: core
  abilities, costs, cooldowns, targeting, effects, status effects

openrpg-quests
  depends on: core
  quest states, objectives, prerequisites, branching outcomes

openrpg-dialogue
  depends on: core
  branching dialogue, conditions, consequences, checks, localization keys

openrpg-factions
  depends on: core
  factions, reputation, hostility, relationship thresholds

openrpg-economy
  depends on: core, inventory
  currencies, vendors, shops, pricing, barter, restocking

openrpg-encounters
  depends on: core
  optional: combat, quests, loot
  encounter definitions, spawn groups, scaling, rewards

openrpg-loot
  depends on: core, inventory
  loot tables, weighted drops, rarity, seeded generation

openrpg-script
  depends on: core
  safe scripting, ORS or embedded sandboxed runtime, designer actions

openrpg-world
  planned
  depends on: core
  optional: quests, encounters
  spatial areas, region state, map/dungeon generation, spatial queries

openrpg-net
  planned
  depends on: core
  optional: combat
  deterministic sync, authoritative server mode, state replication,
  lobby/session hooks, rollback support

openrpg-tools
  depends on: core and selected modules
  validation CLI, simulators, migration checker, trace viewers,
  balance reports, content diagnostics
```

## 9. Module Registration

Each package declares its hard and optional dependencies. The core validates module order and exposes registered systems after boot.

```typescript
interface OpenRPGModule {
  id: string
  version: string
  requires: string[]
  optional: string[]
  register(core: OpenRPGCore): void
}
```

Example:

```typescript
import { OpenRPGCore } from "openrpg-core"
import { CombatModule } from "openrpg-combat"
import { InventoryModule } from "openrpg-inventory"
import { EquipmentModule } from "openrpg-equipment"
import { QuestModule } from "openrpg-quests"

const engine = new OpenRPGCore("openrpg.toml")

engine.use(InventoryModule)
engine.use(EquipmentModule)
engine.use(CombatModule)
engine.use(QuestModule)

await engine.boot()
```

During `register()`, a module may:

- Register content schemas.
- Register components.
- Register commands.
- Register events.
- Register interceptors.
- Register save serializers.
- Register migrations.
- Register validators.
- Register debug trace providers.
- Register tick handlers.

Boot fails if hard dependencies are missing or incompatible.

## 10. Runtime Architecture

OpenRPG is organized into four conceptual layers.

### 10.1 Content Layer

Static game data:

- Item templates.
- Equipment definitions.
- Ability definitions.
- Status effect definitions.
- Character archetypes.
- Enemy templates.
- Quest graphs.
- Dialogue graphs.
- Loot tables.
- Encounter definitions.
- Faction definitions.
- Economy rules.
- Rule configuration.

### 10.2 Rules Layer

Mechanical behavior:

- Formula evaluation.
- Condition checks.
- Damage calculation.
- Hit calculation.
- Status effect ticking.
- Equipment validation.
- Level-up rules.
- Quest objective updates.
- Dialogue option availability.
- Loot generation.
- Price calculation.
- Save migration.

### 10.3 State Layer

Mutable runtime state:

- Entity registry.
- Component values.
- Inventories.
- Equipment state.
- Active combat sessions.
- Active effects.
- Active quests.
- Dialogue sessions and history.
- World flags and variables.
- Reputation values.
- Economy balances and vendor stock.
- RNG state.
- Save metadata.

### 10.4 Integration Layer

Game-facing surface:

- Commands.
- Events.
- State patches.
- Query API.
- Interceptors.
- Plugins.
- Serialization.
- Debug traces.
- Tooling.

The frontend speaks to the Integration Layer and avoids reaching directly into internal system implementation.

## 11. Tick Model

OpenRPG runs on logical ticks. The consuming frontend or host drives ticks by calling the engine. OpenRPG does not own the game loop or run a hidden thread.

```typescript
const tick = engine.tick(deltaMs)
```

Tick result:

```typescript
interface TickResult {
  events: OpenRPGEvent[]
  patches: StatePatch[]
  pendingInputRequests: InputRequest[]
  trace?: RuleTrace[]
}
```

Use cases:

- Turn-based games may call `tick()` when advancing turns or resolving queued effects.
- Real-time games may call `tick(deltaMs)` every frame.
- Servers may call `tick()` on a fixed schedule.
- Simulators may call `tick()` in a tight loop with deterministic inputs.

The tick model must support:

- Fixed-step simulation.
- Variable frontend frame timing.
- Queued commands.
- Status effect timing.
- Cooldowns.
- ATB-style gauges.
- Delayed effects.
- Time-based world events.

## 12. Commands, Events, Patches, and Queries

### 12.1 Commands

Commands are requests to mutate engine state.

Examples:

- `entity.create`
- `inventory.addItem`
- `inventory.transferItem`
- `equipment.equip`
- `equipment.unequip`
- `ability.use`
- `combat.start`
- `combat.performAction`
- `combat.advanceTurn`
- `progression.awardXP`
- `quest.accept`
- `quest.updateObjective`
- `dialogue.start`
- `dialogue.chooseOption`
- `world.setFlag`
- `economy.buyFromVendor`
- `save.create`
- `save.restore`

Command shape:

```typescript
interface Command<TPayload = unknown> {
  id?: string
  type: string
  actorId?: EntityId
  payload: TPayload
  metadata?: Record<string, unknown>
}
```

### 12.2 Command Results

Success:

```typescript
interface CommandSuccess {
  ok: true
  state?: StateSnapshot
  patches: StatePatch[]
  events: OpenRPGEvent[]
  trace?: RuleTrace
}
```

Failure:

```typescript
interface CommandFailure {
  ok: false
  error: EngineError
}
```

Error shape:

```typescript
interface EngineError {
  code: string
  message: string
  details?: Record<string, unknown>
  path?: string
}
```

Example:

```json
{
  "ok": false,
  "error": {
    "code": "ABILITY_ON_COOLDOWN",
    "message": "Firebolt is on cooldown.",
    "details": {
      "abilityId": "mygame:firebolt",
      "remainingTurns": 2
    }
  }
}
```

### 12.3 Events

Events are emitted after resolution. The outcome has already been committed. Events are for frontend reactions, logs, analytics, tooling, quest updates, and downstream systems.

```typescript
engine.on("combat.damageDealt", event => {
  showDamageNumber(event.targetId, event.amount)
  playHitSound(event.damageType)
})
```

Event shape:

```typescript
interface OpenRPGEvent<TPayload = unknown> {
  id: string
  type: string
  tick: number
  source?: EntityId
  target?: EntityId
  payload: TPayload
}
```

### 12.4 Interceptors

Interceptors run before resolution or before commit. They can inspect, modify, deny, or replace pending outcomes.

```typescript
engine.intercept("combat.beforeDamage", (event, next) => {
  if (event.target.tags.includes("god_mode")) {
    return next({ ...event, amount: 0 })
  }
  return next(event)
})
```

Interceptors must be ordered, traceable, deterministic, and registered as trusted plugin behavior rather than arbitrary untrusted content behavior.

### 12.5 State Patches

State patches describe what changed.

Required uses:

- Frontend synchronization.
- Multiplayer replication.
- Debugging.
- Replay.
- Tooling.
- Save diffs.

Patch shape:

```typescript
interface StatePatch {
  op: "add" | "remove" | "replace" | "increment" | "decrement"
  path: string
  value?: unknown
  previous?: unknown
}
```

### 12.6 Queries

Queries read state without mutation.

Examples:

- `engine.query.characterStats(entityId)`
- `engine.query.inventory(entityId)`
- `engine.query.availableAbilities(entityId)`
- `engine.query.questLog(entityId)`
- `engine.query.dialogueOptions(sessionId)`
- `engine.query.worldFlag(key)`
- `engine.query.vendorStock(vendorId)`

Queries should be deterministic, side-effect free, and safe for frontends to call frequently.

## 13. Core Entity and Component Model

OpenRPG uses an entity/component model for runtime objects.

```typescript
interface Entity {
  id: EntityId
  tags: string[]
  components: Record<string, ComponentData>
  metadata?: Record<string, unknown>
}
```

Entities may represent player characters, party members, NPCs, enemies, summons, items, containers, locations, factions, vendors, quest objects, combat arenas, abstract systems, or controllers.

Built-in component families:

- `Identity`
- `Stats`
- `Health`
- `Resources`
- `Inventory`
- `Equipment`
- `Abilities`
- `StatusEffects`
- `Progression`
- `Faction`
- `DialogueState`
- `QuestLog`
- `Position`
- `AIBehavior`
- `Vendor`

Components are plain data. Systems own behavior.

Custom components are registered through schemas:

```typescript
engine.components.define({
  id: "mygame:craftingSkill",
  schema: CraftingSkillSchema
})
```

## 14. Stat, Attribute, and Resource System

The stat system manages numeric and categorical values used by mechanics.

Examples:

- Health.
- Mana.
- Stamina.
- Strength.
- Dexterity.
- Intelligence.
- Armor.
- Accuracy.
- Evasion.
- Speed.
- Fire resistance.
- Carry weight.
- Morale.
- Hunger.
- Sanity.

Capabilities:

- Base values.
- Derived values.
- Current and maximum resources.
- Flat modifiers.
- Percentage modifiers.
- Temporary modifiers.
- Permanent modifiers.
- Caps and floors.
- Formula evaluation.
- Modifier priorities.
- Stacking rules.
- Source tracking.
- Tags for damage, resources, schools, and scaling groups.

Default modifier formula:

```text
final = clamp((base + flatModifiers) * (1 + percentModifiers), floor, cap)
```

Example stat formulas:

```text
maxHealth = 50 + constitution * 8 + level * 5
fireDamage = baseDamage + intelligence * 0.5
evasion = dexterity * 0.5 + speed * 0.2
carryWeight = 20 + strength * 4
```

Games define stat schemas. The engine provides the calculation framework.

## 15. Progression System

The progression system handles growth and unlocks.

Supported models:

- XP levels.
- Skill-use progression.
- Milestone progression.
- Class progression.
- Multiclassing.
- Classless progression.
- Perk trees.
- Skill trees.
- Reputation-gated unlocks.
- Story-gated unlocks.

Responsibilities:

- Award XP.
- Track thresholds.
- Apply level-up effects.
- Grant skill points.
- Validate unlock requirements.
- Track unlocked nodes.
- Emit progression events.
- Recalculate derived stats.

Skill trees are directed acyclic graphs. The engine validates prerequisites and tracks unlock state. The frontend decides how to render the tree.

## 16. Inventory System

The inventory system manages item ownership, storage, transfer, and use.

Capabilities:

- Item templates.
- Unique item instances.
- Stackable items.
- Nested containers.
- Weight limits.
- Volume or slot limits.
- Tags.
- Durability.
- Binding rules.
- Ownership.
- Transfer validation.
- Consumption.
- Item destruction.
- Item instance metadata.

Item template:

```toml
[item]
id = "mygame:potion_small"
name_key = "item.potion_small.name"
desc_key = "item.potion_small.desc"
type = "consumable"
max_stack = 99
tags = ["potion", "healing"]

[[item.effects]]
type = "heal"
formula = "50 + user.level * 5"
target = "self"
```

The inventory system should support simple inventories and complex container graphs.

## 17. Equipment System

The equipment system handles wearable and wieldable items.

Capabilities:

- Equipment slots.
- Slot groups.
- Two-hand rules.
- Requirements.
- Stat modifiers.
- Passive abilities.
- Set bonuses.
- Durability effects.
- Conflicting equipment rules.
- Equip and unequip events.

Equipment layouts are game-defined.

Example:

```toml
[item]
id = "mygame:iron_sword"
name_key = "item.iron_sword.name"
type = "weapon"
max_stack = 1
tags = ["weapon", "melee", "sword"]

[item.equipment]
slots = ["mainHand"]
requirements = { level = 1, strength = 6 }

[item.stats.modifiers]
strength = { flat = 1 }

[item.stats.damage]
physical = 8
```

## 18. Ability and Effect System

Abilities define actions entities can perform.

Ability types:

- Attacks.
- Spells.
- Skills.
- Reactions.
- Passives.
- Movement actions.
- Exploration actions.
- Dialogue actions.
- Crafting actions.

Ability capabilities:

- Costs.
- Cooldowns.
- Charges.
- Targeting.
- Conditions.
- Effects.
- Scaling formulas.
- Tags.
- Availability checks.
- Interrupt rules.
- Resource consumption.
- Trace output.

Effects are mechanical consequences:

- Damage.
- Healing.
- Resource changes.
- Stat modifiers.
- Status effects.
- Summons.
- Forced movement.
- Item creation.
- Item destruction.
- Quest updates.
- World flag changes.
- Dialogue branch changes.

Example:

```toml
[ability]
id = "mygame:firebolt"
name_key = "ability.firebolt.name"
type = "spell"
tags = ["fire", "ranged"]

[ability.cost]
mana = 8

[ability.targeting]
type = "single"
relation = "enemy"

[ability.cooldown]
turns = 1

[[ability.effects]]
type = "damage"
damage_type = "fire"
formula = "10 + intelligence * 0.5"

[[ability.effects]]
type = "status"
status_id = "mygame:burning"
chance = 0.25
duration = 3
```

## 19. Status Effect System

Status effects are ongoing modifiers or behaviors.

Capabilities:

- Duration.
- Infinite duration.
- Stack limits.
- Refresh rules.
- Replace rules.
- Tick timing.
- Source tracking.
- Target tracking.
- Dispel rules.
- Immunities.
- Resistances.
- Periodic effects.
- Expiration events.

Example:

```toml
[status]
id = "mygame:burning"
name_key = "status.burning.name"
tags = ["fire", "dot", "dispellable"]

[status.stacking]
rule = "refresh"
max = 1

[status.tick]
timing = "end_of_turn"
effect = "damage"
damage_type = "fire"
formula = "2 + source.level * 0.1"
```

## 20. Combat System

The combat system resolves conflict between entities.

Supported modes:

- Turn-based.
- Active Time Battle.
- Real-time with pause.
- Real-time discrete action resolution.
- Tactical grid combat.
- Theater-of-the-mind combat.
- Party combat.
- One-on-one combat.

Core responsibilities:

- Start and end combat sessions.
- Track participants.
- Track teams.
- Track initiative or timing gauges.
- Request player input when needed.
- Run AI selection for AI-controlled entities.
- Validate actions.
- Validate targets.
- Resolve hit, miss, dodge, block, crit, damage, healing, and effects.
- Tick status effects.
- Check defeat and victory conditions.
- Generate rewards.
- Emit combat events and traces.

Turn-based example flow:

```text
1. Start combat session with participants.
2. Resolve turn order.
3. Emit combat.turnOrderSet.
4. Current actor begins turn.
5. If player-controlled, emit input request.
6. Frontend submits command.
7. Engine validates action and targets.
8. Engine resolves hit, damage, effects, death checks.
9. Engine emits events and patches.
10. Engine advances to next actor.
11. Combat ends when victory or defeat conditions are met.
```

Pluggable resolvers:

- Initiative resolver.
- Hit chance resolver.
- Critical hit resolver.
- Damage resolver.
- Resistance resolver.
- AI resolver.
- Targeting resolver.
- Victory condition resolver.
- Reward resolver.
- Positioning resolver.

Default first target: a deterministic turn-based resolver. This is easiest to test, simulate, and adapt into other styles.

## 21. Quest System

The quest system tracks player goals and outcomes.

Capabilities:

- Quest definitions.
- Quest states.
- Objectives.
- Objective counters.
- Objective dependencies.
- Prerequisites.
- Branching outcomes.
- Failure conditions.
- Hidden objectives.
- Repeatable quests.
- Timed quests.
- Rewards.
- Event-driven objective updates.

Objective examples:

- Kill entity type.
- Collect item.
- Reach location.
- Talk to NPC.
- Set world flag.
- Survive encounter.
- Craft item.
- Make dialogue choice.
- Complete skill check.

Example:

```toml
[quest]
id = "mygame:lost_sword"
name_key = "quest.lost_sword.name"
start_trigger = { talk_to = "mygame:npc_blacksmith" }

[[quest.objectives]]
id = "collect_ore"
type = "collect"
item = "mygame:iron_ore"
quantity = 5

[[quest.objectives]]
id = "defeat_wolf"
type = "defeat"
tag = "wolf"
quantity = 3
requires = ["collect_ore"]

[quest.rewards]
xp = 200
items = [{ id = "mygame:iron_sword", quantity = 1 }]
```

Quest updates should mostly be driven by engine events, not manual frontend bookkeeping.

## 22. Dialogue System

The dialogue system manages structured conversations.

Capabilities:

- Dialogue sessions.
- Nodes.
- Speaker data.
- Player choices.
- Conditions.
- Consequences.
- Skill checks.
- Relationship checks.
- World flag checks.
- Quest integration.
- Economy integration.
- Localized text keys.
- Dialogue history flags.

The frontend renders text and choices. OpenRPG determines which choices are available and what consequences happen when a choice is selected.

## 23. World State System

The world state system stores global and scoped facts.

Examples:

- A boss is defeated.
- A city is hostile.
- A door is unlocked.
- A faction controls a region.
- A companion has left the party.
- A ritual was completed before midnight.

Capabilities:

- Boolean flags.
- Numeric variables.
- String variables.
- Enumerated variables.
- Scoped state.
- Region state.
- State history where configured.
- Condition evaluation.
- Event emission on change.

## 24. Faction and Reputation System

The faction system tracks relationships between entities, groups, regions, and the player.

Capabilities:

- Faction definitions.
- Membership.
- Reputation values.
- Relationship states.
- Hostility rules.
- Ally and enemy checks.
- Thresholds.
- Reputation modifiers.
- Event-driven reputation changes.
- Dialogue and vendor integration.
- Combat targeting integration.

## 25. Economy and Vendor System

The economy system supports currencies, shops, vendors, trades, and pricing.

Capabilities:

- Multiple currencies.
- Entity balances.
- Vendor inventories.
- Buy and sell pricing.
- Restocking rules.
- Price modifiers.
- Barter hooks.
- Reputation discounts.
- Trade validation.
- Transaction events.

## 26. Encounter System

The encounter system creates structured gameplay situations.

Encounter types:

- Combat encounters.
- Dialogue encounters.
- Exploration encounters.
- Skill challenges.
- Random encounters.
- Boss encounters.
- Scripted sequences.

Capabilities:

- Encounter definitions.
- Spawn groups.
- Conditions.
- Scaling rules.
- Rewards.
- Failure handling.
- Completion state.
- Optional links to quests and world state.

## 27. Loot System

The loot system generates rewards.

Capabilities:

- Loot tables.
- Weighted entries.
- Drop conditions.
- Rarity tiers.
- Quantity ranges.
- Unique item rules.
- Context-aware drops.
- Seeded generation.
- Preview/simulation support.

Example:

```toml
[loot_table]
id = "mygame:wolf_common"

[[loot_table.entries]]
item = "mygame:wolf_pelt"
weight = 70
quantity = { min = 1, max = 2 }

[[loot_table.entries]]
item = "mygame:wolf_fang"
weight = 30
quantity = { min = 1, max = 1 }
```

## 28. Scripting System

The scripting package is optional. It provides designer-authored behavior without requiring engine source changes.

Supported uses:

- Trigger scripts on world events.
- Dialogue consequences.
- Quest consequences.
- Timed sequences.
- Custom condition checks.
- Safe entity manipulation.
- Custom scripted actions.

Preferred direction:

- Provide a small safe expression language for formulas and conditions.
- Provide `openrpg-script` later for larger scripts.
- Consider ORS, OpenRPG Script, as a purpose-built deterministic DSL.
- Consider sandboxed Lua only if deterministic replay and host isolation can be guaranteed.

Security requirements:

- No arbitrary filesystem access.
- No network access.
- Bounded execution.
- Deterministic behavior.
- Explicit host API.
- Traceable script effects.

## 29. World Package

`openrpg-world` is planned after the core RPG systems are stable.

Possible scope:

- Area graphs.
- Regions.
- Spatial queries.
- Procedural dungeon generation.
- Encounter placement.
- Region state.
- Location tags.
- Quest-to-map objective placement hooks.

Non-goals for this package:

- Rendering maps.
- Physics.
- Collision.
- Full editor UI.

## 30. Networking Package

`openrpg-net` is planned after deterministic single-process simulation is proven.

Possible scope:

- Server-authoritative command execution.
- Client command submission.
- State patch broadcast.
- Deterministic sync verification.
- Replay logs.
- Rollback support.
- Lobby and session hooks.
- Authority checks.

Networking transport should be replaceable. OpenRPG should provide deterministic state and validation, not force a single socket or protocol stack.

## 31. Save and Load System

Save state is a complete, versioned snapshot of engine state at a tick.

Captured state:

- Entity registry.
- Component data.
- Inventory contents.
- Equipment state.
- Quest log.
- Dialogue history.
- World flags.
- Faction reputation.
- Economy balances.
- Vendor stock state.
- Active status effects.
- Combat session state.
- Encounter state.
- RNG state.
- Scripting VM state where enabled.
- Save metadata.
- Schema version.

API:

```typescript
const blob = engine.save.snapshot()
engine.save.restore(blob)
```

Partial snapshots:

```typescript
const checkpoint = engine.save.snapshot({
  systems: ["combat", "world"]
})
```

Save loading must validate schema versions and either migrate safely or return a precise error.

## 32. Configuration

OpenRPG should support a single bootstrap config.

Example:

```toml
[engine]
tick_rate_ms = 16
rng_seed = "auto"
strict_validation = true
dev_hot_reload = true

[modules]
combat = true
progression = true
inventory = true
equipment = true
abilities = true
quests = true
dialogue = true
factions = true
economy = true
encounters = true
loot = true
scripting = false

[combat]
mode = "turn_based"

[content]
root = "data/"
formats = ["toml", "yaml", "json"]

[localization]
validate_keys = true
default_locale = "en"
```

Config should be loadable from files or code-native objects.

## 33. Content Model

OpenRPG accepts structured content files and normalizes them into validated internal schemas.

Supported edge formats:

- TOML.
- YAML.
- JSON.
- Code-native object definitions.

Internal content must be schema-validated regardless of file format.

Content categories:

- Stats.
- Resources.
- Damage types.
- Tags.
- Items.
- Equipment.
- Abilities.
- Effects.
- Status effects.
- Character archetypes.
- Enemy templates.
- AI profiles.
- Quests.
- Dialogue.
- Factions.
- Vendors.
- Encounters.
- Loot tables.
- Progression curves.
- Skill trees.
- Rule configuration.

## 34. Namespaced IDs

All content IDs should be namespaced to prevent collisions.

Examples:

```text
core:health
mygame:iron_sword
frostlands:ice_axe
community_magic:arcane_bolt
```

Rules:

- `core:` is reserved for OpenRPG built-ins.
- Games define their own root namespace.
- Mods must use their own namespace.
- Overrides must explicitly target another namespaced ID.
- Validation fails on accidental duplicate final IDs.

## 35. Content Validation

Validation runs at boot and through standalone tooling.

Validation catches:

- Duplicate IDs.
- Missing references.
- Invalid formulas.
- Invalid condition expressions.
- Unknown tags in strict mode.
- Invalid stat names.
- Invalid resource names.
- Invalid damage types.
- Invalid equipment slots.
- Circular skill tree dependencies.
- Circular quest objective dependencies.
- Broken quest graphs.
- Broken dialogue graphs.
- Invalid loot weights.
- Invalid localization keys when enabled.
- Invalid module dependencies.
- Invalid save migrations.
- Schema violations.

Validation errors should be precise and actionable.

Example:

```text
[ERROR] Content validation failed

ability "mygame:firebolt"
  effects[1].status_id references missing status "mygame:burning"
  hint: define the status or fix the namespace

quest "mygame:lost_sword"
  objectives[1].requires references unknown objective "collect_gem"
  defined objectives: ["collect_ore", "defeat_wolf"]
```

## 36. Rules Runtime

The rules runtime evaluates formulas, conditions, and resolver pipelines.

Capabilities:

- Formula evaluation.
- Condition evaluation.
- Rule priority.
- Resolver registration.
- Interceptor pipelines.
- Safe expression execution.
- Trace output.
- Deterministic RNG access.
- Context objects.

Rules should be explainable. A developer should be able to ask why a result occurred.

Example trace:

```text
combat.performAction mygame:firebolt
  actor: entity:hero
  target: entity:wolf_01
  cost:
    mana: -8
  hit:
    base: 70
    actor.accuracy: +12
    target.evasion: -8
    final: 74
    roll: 41
    result: hit
  damage:
    formula: 10 + intelligence * 0.5
    intelligence: 16
    raw: 18
    target.fireResistance: -3
    final: 15
  status:
    mygame:burning
    chance: 0.25
    roll: 0.18
    result: applied
```

## 37. Plugin and Extension Model

OpenRPG supports several extension levels.

### 37.1 Hooks

Hooks observe lifecycle moments.

Examples:

- Before ability use.
- After ability use.
- Before damage.
- After damage.
- Before quest completion.
- After item transfer.
- Before dialogue option availability check.
- After save serialization.

### 37.2 Custom Resolvers

Resolvers replace calculations.

Examples:

- Damage resolver.
- Hit resolver.
- Initiative resolver.
- Loot resolver.
- Price resolver.
- Level-up resolver.
- Quest objective resolver.
- Dialogue condition resolver.

### 37.3 Custom Content Types

Games can register new schemas.

Examples:

- Crafting recipes.
- Spell schools.
- Ship modules.
- Colony rooms.
- Monster taming rules.
- Social influence mechanics.
- Relationship systems.

### 37.4 Middleware

Middleware wraps commands.

Use cases:

- Logging.
- Analytics.
- Multiplayer validation.
- Anti-cheat checks.
- Debug recording.
- Mod integration.
- Permission checks.

### 37.5 Extension Point Summary

| Extension Point | Mechanism |
| --- | --- |
| Custom component | `engine.components.define(schema)` |
| Custom command | `engine.commands.register(type, handler)` |
| Custom event listener | `engine.on(type, listener)` |
| Pre-resolution intercept | `engine.intercept(type, interceptor)` |
| Custom resolver | `engine.resolvers.register(id, fn)` |
| Custom content type | `engine.content.defineType(schema)` |
| Custom formula function | `engine.formulas.registerFunction(name, fn)` |
| Save migration | `engine.save.registerMigration(from, to, fn)` |
| Command middleware | `engine.use(middleware)` |

## 38. Public API Shape

The exact implementation language may vary, but the conceptual API should remain stable.

Engine creation:

```typescript
const engine = createOpenRPG({
  config: "openrpg.toml",
  content: "./data",
  modules: [
    InventoryModule,
    EquipmentModule,
    AbilityModule,
    CombatModule,
    QuestModule
  ],
  seed: "auto"
})

await engine.boot()
```

State creation:

```typescript
const state = engine.createState({
  party,
  world,
  inventory
})
```

Command execution:

```typescript
const result = engine.execute(state, {
  type: "combat.performAction",
  actorId: "entity:hero",
  payload: {
    abilityId: "mygame:firebolt",
    targetIds: ["entity:wolf_01"]
  }
})
```

Tick loop:

```typescript
function gameLoop(deltaMs: number) {
  const tick = engine.tick(deltaMs)

  for (const event of tick.events) {
    frontend.handleEvent(event)
  }

  for (const request of tick.pendingInputRequests) {
    frontend.handleInputRequest(request)
  }

  requestAnimationFrame(gameLoop)
}
```

## 39. Frontend Integration Flow

1. Developer defines content.
2. Developer configures modules.
3. Developer boots the engine.
4. Developer creates or loads state.
5. Frontend subscribes to events.
6. Frontend queries state for UI.
7. Player performs an action.
8. Frontend sends a command.
9. OpenRPG validates and resolves the command.
10. OpenRPG updates state.
11. OpenRPG emits events and patches.
12. Frontend renders the outcome.

The frontend does not duplicate mechanical truth. It presents the truth emitted by OpenRPG.

## 40. Debugging and Tooling

Required tools:

| Tool | Purpose |
| --- | --- |
| Content validator CLI | Validate all content without booting a full game |
| State inspector | Dump and query runtime state |
| Event log viewer | Inspect emitted events |
| Rule trace viewer | Explain command outcomes |
| Combat simulator | Run headless combat fixtures |
| Quest graph validator | Detect broken quest dependencies |
| Dialogue graph validator | Detect missing or unreachable nodes |
| Save migration checker | Validate save files across schema versions |
| Loot simulator | Roll loot tables many times and report distribution |

Nice-to-have tools:

- Visual quest editor.
- Visual dialogue editor.
- Encounter simulator.
- Balance report generator.
- Test scenario runner.
- Mod conflict inspector.
- Content hot reload in dev mode.

## 41. Testing Strategy

OpenRPG must be designed for testability.

Test categories:

- Unit tests for system functions.
- Integration tests for command flows.
- Snapshot tests for deterministic simulations.
- Content validation tests.
- Save/load migration tests.
- Plugin behavior tests.
- Fuzz tests for random command sequences.
- Multiplayer determinism tests.

Example scenarios:

- Equipping armor increases derived defense by the expected amount.
- A quest objective updates when a matching kill event fires.
- A burn effect deals damage at the configured tick timing.
- A loot table with a fixed seed produces stable output.
- A save file created on schema version 1 migrates to version 2.
- A custom interceptor halves fire damage against fire-resistant targets.
- Replaying a command log from the same seed produces identical patches.

## 42. Multiplayer Considerations

OpenRPG should not require multiplayer, but its architecture must enable it.

Enabling properties:

- Deterministic command execution.
- Seeded RNG.
- Serializable state patches.
- Typed command validation.
- Replayable event logs.
- Queryable authoritative state.
- Clear separation between prediction and authority.
- Headless server execution.

`openrpg-net` may eventually provide:

- Server mode.
- Client command submission.
- State diff broadcast.
- Latency compensation hooks.
- Rollback support.
- Session management hooks.
- Combat authority helpers.

The engine itself must avoid nondeterministic hidden behavior that would make netcode unreliable.

## 43. Modding

OpenRPG is mod-friendly by design.

Supported concepts:

- Content packs.
- Namespaced IDs.
- Explicit overrides.
- Dependency ordering.
- Conflict detection.
- Version compatibility.
- Per-pack validation.
- Optional trusted plugin APIs.
- Untrusted data-only mods.

Mod manifest example:

```toml
[mod]
id = "community_magic"
name = "Community Magic"
version = "1.0.0"
requires_engine = ">=0.3.0"
depends_on = ["mygame"]

[content]
root = "content/"
```

Trusted code plugins and untrusted content packs must be treated differently.

## 44. Localization

OpenRPG does not hardcode player-facing strings in content.

Content references localization keys:

- Item names and descriptions.
- Ability names and descriptions.
- Quest names and objective text.
- Dialogue lines and choices.
- Status effect names and descriptions.
- Player-facing error messages.

Example:

```toml
[item]
id = "mygame:iron_sword"
name_key = "item.iron_sword.name"
desc_key = "item.iron_sword.desc"
```

The engine passes localization keys through state and events. The frontend or game localization layer resolves them.

## 45. Accessibility and Player Experience Boundaries

OpenRPG does not render UI, but it should expose structured information that helps frontends build accessible interfaces.

Useful outputs:

- Typed event payloads.
- Machine-readable command errors.
- Structured quest objective state.
- Structured combat logs.
- Rule traces.
- Status effect metadata.
- Localized text keys.
- Input request metadata.

This enables readable combat logs, screen-reader friendly quest journals, clear action feedback, and consistent UI state.

## 46. Security

Security requirements:

- Validate all external content before loading.
- Do not execute arbitrary code from untrusted content.
- Separate trusted engine plugins from untrusted mods.
- Use a safe formula language.
- Do not expose filesystem or network access to scripts by default.
- Bound script execution.
- Make script side effects explicit and traceable.
- Validate save files before loading.

Any dynamic plugin loading must be opt-in and clearly marked as trusted-code execution.

## 47. Versioning

OpenRPG versions these artifacts:

- Engine API.
- Core package.
- Module packages.
- Content schemas.
- Save schemas.
- Plugin API.
- Tooling CLI.

Version policy:

| Bump | Meaning |
| --- | --- |
| Patch | Bug fixes with no public API or schema change |
| Minor | Backward-compatible additions and optional fields |
| Major | Breaking API, schema, save, or plugin changes |

Breaking changes require migration guidance. Save schema changes require migration support or a clear incompatibility error.

## 48. Minimum Viable Product

The first useful OpenRPG release should include:

- `openrpg-core`
- Entity and component system.
- Event bus.
- Interceptor pipeline.
- Command pipeline.
- Tick model.
- Seeded RNG.
- State patches.
- Save/load snapshots.
- Content registry.
- Content validation CLI.
- Stat and resource system.
- Inventory system.
- Equipment system.
- Ability and effect system.
- Status effects.
- Turn-based combat resolver.
- Basic progression.
- Quest system.
- World flags.
- Rule trace output.
- Basic integration documentation.

The MVP should prioritize correctness, deterministic behavior, clear API contracts, and developer usability over breadth.

## 49. Development Milestones

### Milestone 1: Core Skeleton

- Define monorepo structure.
- Implement `openrpg-core`.
- Implement module registration.
- Implement entity registry.
- Implement component schema registration.
- Implement event bus.
- Implement interceptor pipeline.
- Implement command/result API.
- Implement tick model.
- Implement seeded RNG.
- Implement validation foundation.

### Milestone 2: Content and State

- Implement content registry.
- Implement TOML/YAML/JSON loading.
- Implement namespaced IDs.
- Implement schema validation.
- Implement state snapshots.
- Implement state patches.
- Implement save/load baseline.

### Milestone 3: Stats, Inventory, and Equipment

- Implement stat schemas.
- Implement modifier resolution.
- Implement resources.
- Implement item templates and instances.
- Implement containers and stacks.
- Implement equipment slots.
- Implement stat modifiers from equipment.

### Milestone 4: Abilities, Effects, and Statuses

- Implement ability definitions.
- Implement costs and cooldowns.
- Implement targeting validation.
- Implement effect resolution.
- Implement status effect lifecycle.
- Implement trace output.

### Milestone 5: Combat

- Implement turn-based combat sessions.
- Implement initiative.
- Implement action submission.
- Implement hit and damage defaults.
- Implement victory and defeat conditions.
- Implement combat rewards.
- Implement combat simulator fixtures.

### Milestone 6: Progression, Quests, and World State

- Implement XP and levels.
- Implement skill tree validation.
- Implement quest states.
- Implement event-driven objective updates.
- Implement world flags.
- Implement quest graph validation.

### Milestone 7: Dialogue, Factions, Economy, Encounters, and Loot

- Implement dialogue sessions.
- Implement dialogue graph validation.
- Implement faction reputation.
- Implement vendor inventory and pricing.
- Implement encounter definitions.
- Implement loot tables.
- Implement loot simulator.

### Milestone 8: Tooling and Migration

- Implement validation CLI.
- Implement state inspector.
- Implement event log viewer.
- Implement save migration checker.
- Implement balance report basics.
- Implement test scenario runner.

### Milestone 9: Plugin and Modding API

- Stabilize plugin API.
- Implement custom content type registration.
- Implement custom resolver registration.
- Implement mod manifests.
- Implement content pack dependency ordering.
- Implement conflict detection.

### Milestone 10: Planned Advanced Packages

- Explore `openrpg-script`.
- Explore `openrpg-world`.
- Explore `openrpg-net`.
- Add official bindings beyond the reference implementation.

## 50. Roadmap

Suggested version roadmap:

| Version | Scope |
| --- | --- |
| 0.1.0 | Spec finalized, core architecture prototype |
| 0.2.0 | Entities, components, command/event/tick model |
| 0.3.0 | Content registry, validation, save/load |
| 0.4.0 | Stats, inventory, equipment, abilities |
| 0.5.0 | Turn-based combat, effects, statuses |
| 0.6.0 | Progression, quests, world flags |
| 0.7.0 | Dialogue, factions, economy, loot, encounters |
| 0.8.0 | Tooling, simulators, migration checker |
| 0.9.0 | Plugin API, modding, content packs |
| 1.0.0 | Stable core API and schema compatibility promise |

## 51. Success Criteria

OpenRPG is successful when:

- A developer can build a small RPG prototype without writing custom inventory, stats, quests, and combat infrastructure.
- The same core supports at least two meaningfully different RPG styles.
- Game-specific rules can be extended without changing engine internals.
- Content errors are caught before runtime with actionable messages.
- Save/load works reliably across schema versions.
- Combat and quest flows can be tested without a rendered frontend.
- A frontend can integrate through commands, queries, events, and patches.
- A server can run mechanics headlessly.
- A mod can add items, abilities, quests, and encounters without forking the base game.
- Replaying a seeded command log produces identical state patches.

## 52. Open Questions

These decisions should be resolved before implementation begins:

- Should the first reference implementation be Rust-first, TypeScript-first, or dual-track?
- Which binding should be official first: TypeScript, Python, or GDScript?
- Should the engine accept all TOML/YAML/JSON from day one, or pick one default first?
- Should formulas use a custom expression language only, or should `openrpg-script` ship earlier?
- Should save files use snapshots only at first, or snapshot plus event log?
- Should dynamic plugin loading be supported, or should all code plugins be statically registered by the game?
- How should ATB timing resolve ties and sub-tick ordering?
- What is the minimal stable content schema for `1.0.0`?
- Which parts of `openrpg-world` belong in core as abstract position data, and which belong in the planned world package?
- What guarantees should `openrpg-net` make about prediction, rollback, and server authority?

## 53. Recommended Initial Direction

Start with a deterministic, headless, modular core. Use a Rust-first reference implementation if the project prioritizes long-term performance, bindings, and server use. Use TypeScript-first only if the project prioritizes rapid web-facing iteration and tooling over low-level guarantees.

The first implementation should not try to solve every RPG style at once. It should build the reliable nucleus:

- Core module system.
- Entity/component data model.
- Command/event/interceptor pipeline.
- Deterministic RNG.
- State patches.
- Save snapshots.
- Content validation.
- Stats.
- Inventory.
- Equipment.
- Abilities.
- Effects.
- Turn-based combat.
- Quests.
- World flags.

Once that foundation is stable, advanced systems like scripting, world generation, and networking can become opt-in packages without destabilizing the core.
