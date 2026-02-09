use crate::actions::ProviderError;
use aws_config::{load_defaults,BehaviorVersion};
use log::{info, debug, error};

use aws_sdk_sts::Client as STSClient;
use aws_sdk_ec2::Client as EC2Client;
use aws_sdk_ssm::Client as SSMClient;
use aws_sdk_ssm::error::SdkError;
use aws_sdk_ssm::types::ParameterType;

use crate::responses::*;

#[derive(Debug)]
pub struct AwsProvider {}


impl AwsProvider {
    pub fn new() -> Self {
        AwsProvider {}
    }

    pub async fn who_am_i(&self) -> Result<UserResponse, ProviderError> {
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

        Ok(UserResponse {
            account: response.account().unwrap_or("<unknown>").to_string(),
            arn: response.arn().unwrap_or("<unknown>").to_string(),
            user_id: response.user_id().unwrap_or("<unknkown>").to_string(),
        })
    }

    pub async fn list_instances(&self) -> Result<InstanceResponse, ProviderError> {
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

        // Prepare object that will be returned
        let mut instance_data: InstanceResponse = InstanceResponse::new();

        debug!("Processing instances...");
        for reservation in response.reservations() {
            for instance in reservation.instances() {
                let mut name_tag: String = "<unknown>".to_string();

                // obtain instance name
                debug!("Obtaining instance name");
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

                //TODO obtain fallback tag (configurable from yaml file)

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

                debug!("Parsing instance id");
                let parsed_id = &instance.instance_id().unwrap_or("<unknown>");

                debug!("Parsing private_ip");
                let parsed_private_ip = &instance.private_ip_address().unwrap_or("<unknown>");

                let current_instance = InstanceData {
                    name: name_tag,
                    instance_id: parsed_id.to_string(),
                    state: parsed_state,
                    private_ip: parsed_private_ip.to_string(),
                };
                
                debug!("Appending instance data for {}", &parsed_id);
                instance_data.push(current_instance);
            }
        }
        Ok(instance_data)
    }

    pub async fn list_parameters(&self, param_path: Option<String>, decrypt: bool) -> Result<ParameterResponse, ProviderError> {
        info!("Listing AWS SSM parameters...");

        debug!("Creating SSM client");
        let config = load_defaults(BehaviorVersion::latest()).await;
        let client = SSMClient::new(&config);

        debug!("Obtaining ssm parameters");
        let response = client.get_parameters_by_path()
            .path(param_path.unwrap_or("/".to_string()))
            .recursive(true)
            .with_decryption(decrypt)
            .send()
            .await
            .map_err(|e| {
                match e {
                    SdkError::DispatchFailure(_) => {
                        return ProviderError::ConnectionError;
                    },
                    SdkError::TimeoutError(_) => {
                        return ProviderError::TimeoutError;
                    },
                    _ => {
                        return ProviderError::GeneralError(format!("Failed to get SSM parameters: {}", e));
                    }

                }
            }
            );
        
        debug!("SSM parameters obtained successfully");
        let parsed_data: ParameterResponse = response.iter()
            .flat_map(|page| page.parameters()) // FIXME possible empty iterator, and non-handled errors
            .map(|param| {
                let mut parsed_value: String = String::new(); //FIXME try rewrite without mut
                if param.r#type() == Some(&ParameterType::SecureString) && decrypt {
                   parsed_value = param.value().unwrap_or("<unknown>").to_string(); 
                } else if param.r#type() == Some(&ParameterType::SecureString) && !decrypt {
                   parsed_value = "<encrypted>".to_string();
                } else {
                   parsed_value = param.value().unwrap_or("<unknown>").to_string();
                }

                ParameterData {
                    name: param.name().unwrap_or("<unknown>").to_string(),
                    r#type: param.r#type().map(|t| t.as_str().to_string()).unwrap_or("?".to_string()),
                    value: parsed_value,
                }
            })
            .collect();

        debug!("Parsed SSM parameters successfully");
        Ok(parsed_data)
    }
}

