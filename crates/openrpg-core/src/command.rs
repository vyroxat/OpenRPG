use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use serde_json::Value;

use crate::engine::EngineError;
use crate::event::OpenRpgEvent;
use crate::patch::StatePatch;

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    command_type: String,
    payload: Value,
}

impl Command {
    pub fn new(command_type: impl Into<String>, payload: Value) -> Self {
        Self {
            command_type: command_type.into(),
            payload,
        }
    }

    pub fn command_type(&self) -> &str {
        &self.command_type
    }

    pub fn payload(&self) -> &Value {
        &self.payload
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandOutcome;

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSuccess {
    patches: Vec<StatePatch>,
    events: Vec<OpenRpgEvent>,
}

impl CommandSuccess {
    pub(crate) fn new(patches: Vec<StatePatch>, events: Vec<OpenRpgEvent>) -> Self {
        Self { patches, events }
    }

    pub fn patches(&self) -> &[StatePatch] {
        &self.patches
    }

    pub fn events(&self) -> &[OpenRpgEvent] {
        &self.events
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandResult {
    Success(CommandSuccess),
}

pub struct CommandContext<'a> {
    pub(crate) tick: u64,
    pub(crate) next_event_id: &'a mut u64,
    pub(crate) world: &'a mut BTreeMap<String, Value>,
    pub(crate) events: &'a mut VecDeque<OpenRpgEvent>,
    pub(crate) patches: &'a mut Vec<StatePatch>,
}

impl CommandContext<'_> {
    pub fn set_world_value(&mut self, key: &str, value: Value) {
        let previous = self.world.insert(key.to_string(), value.clone());
        self.patches.push(StatePatch::replace(
            format!("/world/{key}"),
            value,
            previous,
        ));
    }

    pub fn emit(&mut self, event_type: impl Into<String>, payload: Value) {
        let id = format!("event:{}", *self.next_event_id);
        *self.next_event_id += 1;
        self.events
            .push_back(OpenRpgEvent::new(id, event_type, self.tick, payload));
    }
}

pub(crate) type CommandHandler = Arc<
    dyn for<'a> Fn(&mut CommandContext<'a>, &Command) -> Result<CommandOutcome, EngineError>
        + Send
        + Sync,
>;

#[derive(Default)]
pub(crate) struct CommandRegistry {
    handlers: BTreeMap<String, CommandHandler>,
}

impl CommandRegistry {
    pub(crate) fn get(&self, command_type: &str) -> Option<CommandHandler> {
        self.handlers.get(command_type).cloned()
    }
}

pub struct CommandRegistryMut<'a> {
    pub(crate) registry: &'a mut CommandRegistry,
}

impl CommandRegistryMut<'_> {
    pub fn register(
        &mut self,
        command_type: impl Into<String>,
        handler: impl for<'a> Fn(
            &mut CommandContext<'a>,
            &Command,
        ) -> Result<CommandOutcome, EngineError>
        + Send
        + Sync
        + 'static,
    ) {
        self.registry
            .handlers
            .insert(command_type.into(), Arc::new(handler));
    }
}
