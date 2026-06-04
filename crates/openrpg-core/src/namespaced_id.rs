use serde::{Deserialize, Serialize};

use crate::engine::EngineError;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NamespacedId(String);

impl NamespacedId {
    pub fn parse(input: impl Into<String>) -> Result<Self, EngineError> {
        let input = input.into();
        let Some((namespace, value)) = input.split_once(':') else {
            return Err(invalid_id(input));
        };

        if namespace.is_empty() || value.is_empty() {
            return Err(invalid_id(input));
        }

        if !is_id_part(namespace) || !is_id_part(value) {
            return Err(invalid_id(input));
        }

        Ok(Self(input))
    }

    pub fn namespace(&self) -> &str {
        self.0
            .split_once(':')
            .map(|(namespace, _)| namespace)
            .unwrap_or_default()
    }

    pub fn value(&self) -> &str {
        self.0
            .split_once(':')
            .map(|(_, value)| value)
            .unwrap_or_default()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NamespacedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn invalid_id(input: String) -> EngineError {
    EngineError::validation(
        "INVALID_NAMESPACED_ID",
        format!("id {input:?} must use namespace:value with alphanumeric, '_', '-', or '.' parts"),
    )
}

fn is_id_part(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
}
