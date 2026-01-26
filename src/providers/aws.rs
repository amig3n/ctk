use crate::actions::{ProviderActions, ProviderError};
use log::{info, debug, warn, error};

use aws_sdk_sts::Client as STSClient;
use aws_sdk_ec2::Client as EC2Client;

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
        let client = STSClient::new(&config);

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

    pub async fn list_instances(&self) -> Result<(), ProviderError> {
        info!("Listing AWS instances...");

        debug!("Creating EC2 client...");
        let config = aws_config::load_from_env().await;
        let ec2_client = EC2Client::new(&config);

        debug!("Obtaining data about EC2 instances...");
        let response = ec2_client.describe_instances()
            .send()
            .await
            .map_err(|e| {
                error!("Failed to describe instances: {}", e);
                ProviderError::GeneralError(format!("Failed to describe instances: {}", e))
            }
            )?;
        debug!("Data about EC2 instances obtained successfully.");

        debug!("Processing instances...");
        for reservation in response.reservations() {
            for instance in reservation.instances() {
                // obtain instance name
                debug!("Obtaining instance name");
                let mut name_tag: String = "<unknown>".to_string();
                for tag in instance.tags() {
                    if tag.key == Some("Name".to_string()) {
                        debug!("Found Name tag");
                        name_tag = match &tag.value {
                            Some(value) => value.to_string(),
                            None => "<unknown>".to_string(),
                        };
                        break;
                    }
                }

                debug!("Parsing instance state...");
                let parsed_state = match &instance.state() {
                    Some(s) => {
                        match &s.name {
                            Some(name) => name.as_str().to_string(),
                            None => "<unknown>".to_string(),
                        }
                    },
                    None => "<unknown>".to_string(),
                    
                };

                // FIXME structured return for table
                debug!("Printing instances table");
                println!("{} | {} | {} | {}", 
                    name_tag,
                    &instance.instance_id().unwrap_or("<unknown>"),
                    parsed_state,
                    &instance.private_ip_address().unwrap_or("<unknown>"),
                );
            }
        }
        Ok(())
    }
}

