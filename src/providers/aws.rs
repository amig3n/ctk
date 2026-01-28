use crate::actions::ProviderError;
use aws_config::{load_defaults,BehaviorVersion};
use log::{info, debug, error};

use aws_sdk_sts::Client as STSClient;
use aws_sdk_ec2::Client as EC2Client;
use aws_sdk_ssm::Client as SSMClient;

#[derive(Debug)]
pub struct AwsProvider {}

#[derive(Debug)]
pub struct Ec2Instance {
    pub name: String,
    pub instance_id: String,
    pub state: String,
    pub private_ip: String,
}

#[derive(Debug)]
pub struct Ec2Response {
    pub instances: Vec<Ec2Instance>,
}

impl Ec2Response {
    pub fn new() -> Self {
        Ec2Response {
            instances: Vec::new(),
        }
    }

    pub fn push(&mut self, instance: Ec2Instance) {
        self.instances.push(instance);
    }
}

impl FromIterator<Ec2Instance> for Ec2Response {
    fn from_iter<I: IntoIterator<Item = Ec2Instance>>(iter: I) -> Self {
        let instances: Vec<Ec2Instance> = iter.into_iter().collect();
        Ec2Response { instances }
    }

}

#[derive(Debug)]
pub struct SsmParameter {
    pub name: String,
    pub r#type: String,
    pub value: String,
}

#[derive(Debug)]
pub struct SsmResponse {
    pub parameters: Vec<SsmParameter>,
}

impl FromIterator<SsmParameter> for SsmResponse {
    fn from_iter<I: IntoIterator<Item = SsmParameter>>(iter: I) -> Self {
        let parameters: Vec<SsmParameter> = iter.into_iter().collect();
        SsmResponse { parameters }
    }

}


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

    pub async fn list_instances(&self) -> Result<Ec2Response, ProviderError> {
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
        let mut instance_data: Ec2Response = Ec2Response::new();

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

                debug!("Parsing instnace id");
                let parsed_id = &instance.instance_id().unwrap_or("<unknown>");

                debug!("Parsing private_ip");
                let parsed_private_ip = &instance.private_ip_address().unwrap_or("<unknown>");

                let current_instance = Ec2Instance {
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

    pub async fn list_parameters(&self, param_path: Option<String>, decrypt: bool) -> Result<SsmResponse, ProviderError> {
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
            .await;

        debug!("SSM parameters obtained successfully");
        let parsed_data: SsmResponse = response.iter()
            .flat_map(|page| page.parameters())
            .map(|param| {
                SsmParameter {
                    name: param.name().unwrap_or("<unknown>").to_string(),
                    r#type: param.r#type().map(|t| t.as_str().to_string()).unwrap_or("?".to_string()),
                    value:param.value().unwrap_or("<unknown>").to_string(),
                }
            })
            .collect();

        debug!("Parsed SSM parameters successfully");
        Ok(parsed_data)
    }
}

