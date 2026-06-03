use std::collections::BTreeMap;
use std::sync::Arc;

use crate::command::Command;
use crate::engine::EngineError;

#[derive(Clone, Debug, PartialEq)]
pub enum InterceptorDecision {
    Continue,
    Deny(EngineError),
}

impl InterceptorDecision {
    pub fn deny(error: EngineError) -> Self {
        Self::Deny(error)
    }
}

pub(crate) type InterceptorHandler = Arc<dyn Fn(&Command) -> InterceptorDecision + Send + Sync>;

#[derive(Default)]
pub(crate) struct InterceptorRegistry {
    before_command: BTreeMap<String, Vec<InterceptorHandler>>,
}

impl InterceptorRegistry {
    pub(crate) fn handlers_for(&self, command_type: &str) -> Vec<InterceptorHandler> {
        self.before_command
            .get(command_type)
            .cloned()
            .unwrap_or_default()
    }
}

pub struct InterceptorRegistryMut<'a> {
    pub(crate) registry: &'a mut InterceptorRegistry,
}

impl InterceptorRegistryMut<'_> {
    pub fn before_command(
        &mut self,
        command_type: impl Into<String>,
        handler: impl Fn(&Command) -> InterceptorDecision + Send + Sync + 'static,
    ) {
        self.registry
            .before_command
            .entry(command_type.into())
            .or_default()
            .push(Arc::new(handler));
    }
}
