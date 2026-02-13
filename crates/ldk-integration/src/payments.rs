//! Lightning payment handling

use ulw_core::Result;

pub struct PaymentHandler {
    // Placeholder
}

impl PaymentHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn create_invoice(&self, amount_msat: u64, _description: String) -> Result<String> {
        Ok(format!("lnbc{}...", amount_msat))
    }

    pub async fn pay_invoice(&self, _invoice: String) -> Result<()> {
        Ok(())
    }
}
