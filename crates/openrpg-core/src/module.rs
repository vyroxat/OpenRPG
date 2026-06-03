#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleDescriptor {
    id: String,
    version: String,
    requires: Vec<String>,
    optional: Vec<String>,
}

impl ModuleDescriptor {
    pub fn new(id: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            requires: Vec::new(),
            optional: Vec::new(),
        }
    }

    pub fn requires(mut self, id: impl Into<String>) -> Self {
        self.requires.push(id.into());
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn required_modules(&self) -> &[String] {
        &self.requires
    }

    pub fn optional_modules(&self) -> &[String] {
        &self.optional
    }
}
