use crate::actions::{ProviderActions, ProviderError};
use log::{info, debug, warn, error};

use aws_sdk_sts::Client;

#[derive(Debug)]
pub struct AwsProvider {}

impl AwsProvider {
    pub fn new() -> Self {
        AwsProvider {}
    }

    pub async fn who_am_i(&self) -> Result<(), ProviderError> {
        info!("Fetching AWS identity...");

        // Create AWS SDK client
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        //execute get-caller-identity method
        let response = client.get_caller_identity()
            .send()
            .await
            .map_err(|e| {
            error!("Failed to get caller identity: {}", e);
            ProviderError::AuthenticationError
        })?;

        // FIXME consider returning a struct instead of printing directly
        println!("AWS Account: {}", response.account().unwrap_or("Unknown"));
        println!("AWS UserId: {}", response.user_id().unwrap_or("Unknown"));
        println!("AWS ARN: {}", response.arn().unwrap_or("Unknown"));
        Ok(())
    }
}

