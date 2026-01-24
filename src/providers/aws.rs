use crate::actions::{ProviderActions, ProviderError};

#[derive(Debug)]
pub struct AwsProvider {}

impl AwsProvider {
    pub fn new() -> Self {
        AwsProvider {}
    }
}

impl ProviderActions for AwsProvider {
    fn who_am_i(&self) -> Result<String, ProviderError> {
        // Placeholder implementation
        Ok("example-user".to_string())
    }
}

