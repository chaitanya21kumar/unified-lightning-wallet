//! Lightning event handling

use ulw_core::Result;

pub struct EventHandler {
    // Placeholder
}

impl EventHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn process_events(&self) -> Result<()> {
        Ok(())
    }
}
