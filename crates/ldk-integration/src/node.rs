//! LDK node implementation

use ulw_core::{Error, Result};

pub struct LdkNode {
    // Placeholder
}

impl LdkNode {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn node_id(&self) -> String {
        "placeholder_node_id".to_string()
    }
}
