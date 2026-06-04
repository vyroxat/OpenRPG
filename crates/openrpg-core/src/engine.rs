use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde_json::Value;
use thiserror::Error;

use crate::command::{
    Command, CommandContext, CommandRegistry, CommandRegistryMut, CommandResult, CommandSuccess,
};
use crate::component::{ComponentRegistry, ComponentRegistryMut};
use crate::content::{ContentEntry, ContentRegistry, ContentRegistryMut, ContentRegistryRef};
use crate::entity::{EntityRegistry, EntityRegistryMut, EntityRegistryRef};
use crate::event::OpenRpgEvent;
use crate::interceptor::{InterceptorDecision, InterceptorRegistry, InterceptorRegistryMut};
use crate::module::ModuleDescriptor;
use crate::patch::StatePatch;
use crate::rng::DeterministicRng;

#[derive(Clone, Debug)]
pub struct EngineConfig {
    seed: u64,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self { seed: 0 }
    }
}

impl EngineConfig {
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("{code}: {message}")]
pub struct EngineError {
    code: String,
    message: String,
}

impl EngineError {
    pub fn validation(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn forbidden(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub(crate) fn internal(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TickResult {
    tick: u64,
    delta_ms: u64,
    events: Vec<OpenRpgEvent>,
    patches: Vec<StatePatch>,
}

impl TickResult {
    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn delta_ms(&self) -> u64 {
        self.delta_ms
    }

    pub fn events(&self) -> &[OpenRpgEvent] {
        &self.events
    }

    pub fn patches(&self) -> &[StatePatch] {
        &self.patches
    }
}

pub struct OpenRpgCore {
    booted: bool,
    tick: u64,
    next_event_id: u64,
    rng: DeterministicRng,
    modules: Vec<ModuleDescriptor>,
    module_order: Vec<String>,
    content: ContentRegistry,
    components: ComponentRegistry,
    entities: EntityRegistry,
    commands: CommandRegistry,
    interceptors: InterceptorRegistry,
    world: BTreeMap<String, Value>,
    events: VecDeque<OpenRpgEvent>,
    patches: Vec<StatePatch>,
}

impl OpenRpgCore {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            booted: false,
            tick: 0,
            next_event_id: 0,
            rng: DeterministicRng::seeded(config.seed),
            modules: Vec::new(),
            module_order: Vec::new(),
            content: ContentRegistry::default(),
            components: ComponentRegistry::default(),
            entities: EntityRegistry::default(),
            commands: CommandRegistry::default(),
            interceptors: InterceptorRegistry::default(),
            world: BTreeMap::new(),
            events: VecDeque::new(),
            patches: Vec::new(),
        }
    }

    pub fn register_module(&mut self, module: ModuleDescriptor) {
        self.modules.push(module);
    }

    pub fn boot(&mut self) -> Result<(), EngineError> {
        let ids = self
            .modules
            .iter()
            .map(|module| module.id().to_string())
            .collect::<BTreeSet<_>>();

        for module in &self.modules {
            for required in module.required_modules() {
                if !ids.contains(required) {
                    return Err(EngineError::validation(
                        "MODULE_DEPENDENCY_MISSING",
                        format!("module {} requires missing module {required}", module.id()),
                    ));
                }
            }
        }

        self.module_order = sort_modules(&self.modules)?;
        self.booted = true;
        Ok(())
    }

    pub fn is_booted(&self) -> bool {
        self.booted
    }

    pub fn module_order(&self) -> Vec<&str> {
        self.module_order.iter().map(String::as_str).collect()
    }

    pub fn components_mut(&mut self) -> ComponentRegistryMut<'_> {
        ComponentRegistryMut {
            registry: &mut self.components,
        }
    }

    pub fn content_mut(&mut self) -> ContentRegistryMut<'_> {
        ContentRegistryMut {
            registry: &mut self.content,
        }
    }

    pub fn content(&self) -> ContentRegistryRef<'_> {
        ContentRegistryRef {
            registry: &self.content,
        }
    }

    pub fn entities(&self) -> EntityRegistryRef<'_> {
        EntityRegistryRef {
            registry: &self.entities,
        }
    }

    pub fn entities_mut(&mut self) -> EntityRegistryMut<'_> {
        EntityRegistryMut {
            registry: &mut self.entities,
            components: &self.components,
            patches: &mut self.patches,
        }
    }

    pub fn commands_mut(&mut self) -> CommandRegistryMut<'_> {
        CommandRegistryMut {
            registry: &mut self.commands,
        }
    }

    pub fn interceptors_mut(&mut self) -> InterceptorRegistryMut<'_> {
        InterceptorRegistryMut {
            registry: &mut self.interceptors,
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<CommandResult, EngineError> {
        for interceptor in self.interceptors.handlers_for(command.command_type()) {
            match interceptor(&command) {
                InterceptorDecision::Continue => {}
                InterceptorDecision::Deny(error) => return Err(error),
            }
        }

        let handler = self.commands.get(command.command_type()).ok_or_else(|| {
            EngineError::validation(
                "COMMAND_UNKNOWN",
                format!("command {} has not been registered", command.command_type()),
            )
        })?;

        let patch_start = self.patches.len();
        let event_start = self.events.len();
        let mut context = CommandContext {
            tick: self.tick,
            next_event_id: &mut self.next_event_id,
            world: &mut self.world,
            events: &mut self.events,
            patches: &mut self.patches,
        };

        handler(&mut context, &command)?;

        let patches = self.patches[patch_start..].to_vec();
        let events = self.events.iter().skip(event_start).cloned().collect();
        Ok(CommandResult::Success(CommandSuccess::new(patches, events)))
    }

    pub fn tick(&mut self, delta_ms: u64) -> TickResult {
        self.tick += 1;
        TickResult {
            tick: self.tick,
            delta_ms,
            events: self.events.drain(..).collect(),
            patches: self.drain_patches(),
        }
    }

    pub fn drain_patches(&mut self) -> Vec<StatePatch> {
        std::mem::take(&mut self.patches)
    }

    pub fn world_value(&self, key: &str) -> Option<&Value> {
        self.world.get(key)
    }

    pub fn set_world_value(&mut self, key: &str, value: Value) {
        let previous = self.world.insert(key.to_string(), value.clone());
        self.patches.push(StatePatch::replace(
            format!("/world/{key}"),
            value,
            previous,
        ));
    }

    pub fn current_tick(&self) -> u64 {
        self.tick
    }

    pub fn snapshot(&self) -> Result<StateSnapshot, EngineError> {
        Ok(StateSnapshot {
            schema_version: 1,
            tick: self.tick,
            world: self.world.clone(),
            content: self.content.entries(),
            entities: self.entities.entries(),
        })
    }

    pub fn restore_snapshot(&mut self, snapshot: StateSnapshot) -> Result<(), EngineError> {
        if snapshot.schema_version != 1 {
            return Err(EngineError::validation(
                "SAVE_SCHEMA_UNSUPPORTED",
                format!("save schema {} is not supported", snapshot.schema_version),
            ));
        }

        self.tick = snapshot.tick;
        self.world = snapshot.world;
        self.content = ContentRegistry::restore(snapshot.content);
        self.entities = EntityRegistry::restore(snapshot.entities);
        self.events.clear();
        self.patches.clear();
        Ok(())
    }

    pub fn rng_mut(&mut self) -> &mut DeterministicRng {
        &mut self.rng
    }
}

pub type OpenRPGCore = OpenRpgCore;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StateSnapshot {
    schema_version: u32,
    tick: u64,
    world: BTreeMap<String, Value>,
    content: BTreeMap<String, ContentEntry>,
    entities: BTreeMap<crate::entity::EntityId, crate::entity::Entity>,
}

impl StateSnapshot {
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }
}

fn sort_modules(modules: &[ModuleDescriptor]) -> Result<Vec<String>, EngineError> {
    let by_id = modules
        .iter()
        .map(|module| (module.id().to_string(), module))
        .collect::<BTreeMap<_, _>>();
    let mut ordered = Vec::new();
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();

    for module in modules {
        visit_module(
            module.id(),
            &by_id,
            &mut visiting,
            &mut visited,
            &mut ordered,
        )?;
    }

    Ok(ordered)
}

fn visit_module(
    id: &str,
    by_id: &BTreeMap<String, &ModuleDescriptor>,
    visiting: &mut BTreeSet<String>,
    visited: &mut BTreeSet<String>,
    ordered: &mut Vec<String>,
) -> Result<(), EngineError> {
    if visited.contains(id) {
        return Ok(());
    }
    if !visiting.insert(id.to_string()) {
        return Err(EngineError::validation(
            "MODULE_DEPENDENCY_CYCLE",
            format!("module dependency cycle includes {id}"),
        ));
    }

    let module = by_id
        .get(id)
        .ok_or_else(|| EngineError::validation("MODULE_UNKNOWN", format!("unknown module {id}")))?;
    for required in module.required_modules() {
        visit_module(required, by_id, visiting, visited, ordered)?;
    }

    visiting.remove(id);
    visited.insert(id.to_string());
    ordered.push(id.to_string());
    Ok(())
}
