//! Core mechanics nucleus for OpenRPG.

pub mod command;
pub mod component;
pub mod content;
pub mod engine;
pub mod entity;
pub mod event;
pub mod interceptor;
pub mod module;
pub mod namespaced_id;
pub mod patch;
pub mod rng;

pub use command::{Command, CommandContext, CommandOutcome, CommandResult};
pub use component::ComponentSchema;
pub use content::{ContentEntry, ContentPack};
pub use engine::{EngineConfig, EngineError, OpenRPGCore, OpenRpgCore, StateSnapshot, TickResult};
pub use entity::{Entity, EntityId};
pub use event::OpenRpgEvent;
pub use interceptor::InterceptorDecision;
pub use module::ModuleDescriptor;
pub use namespaced_id::NamespacedId;
pub use patch::{PatchOp, StatePatch};
pub use rng::DeterministicRng;
