//! Core mechanics nucleus for OpenRPG.

mod command;
mod component;
mod engine;
mod entity;
mod event;
mod interceptor;
mod module;
mod patch;
mod rng;

pub use command::{Command, CommandContext, CommandOutcome, CommandResult};
pub use component::ComponentSchema;
pub use engine::{EngineConfig, EngineError, OpenRpgCore, TickResult};
pub use entity::{Entity, EntityId};
pub use event::OpenRpgEvent;
pub use interceptor::InterceptorDecision;
pub use module::ModuleDescriptor;
pub use patch::{PatchOp, StatePatch};
pub use rng::DeterministicRng;
