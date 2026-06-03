use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PatchOp {
    Add,
    Remove,
    Replace,
    Increment,
    Decrement,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatePatch {
    op: PatchOp,
    path: String,
    value: Option<Value>,
    previous: Option<Value>,
}

impl StatePatch {
    pub fn add(path: impl Into<String>, value: Value) -> Self {
        Self {
            op: PatchOp::Add,
            path: path.into(),
            value: Some(value),
            previous: None,
        }
    }

    pub fn replace(path: impl Into<String>, value: Value, previous: Option<Value>) -> Self {
        Self {
            op: PatchOp::Replace,
            path: path.into(),
            value: Some(value),
            previous,
        }
    }

    pub fn op(&self) -> PatchOp {
        self.op.clone()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }

    pub fn previous(&self) -> Option<&Value> {
        self.previous.as_ref()
    }
}
