use crate::actions::*;
use crate::responses::*;
use async_trait::async_trait;

#[derive(Debug)]
pub struct CloudflareProvider {}

impl CloudflareProvider {
    pub fn new() -> CloudflareProvider {
        CloudflareProvider{}
    }
}

#[async_trait]
impl ProviderActions for CloudflareProvider {

    async fn who_am_i(&self) -> Result<UserResponse, ProviderError> {
        Ok(UserResponse {
            user_id: "00000000".to_string(),
            arn: "00000000000".to_string(),
            account: "Mocked CF".to_string() 
        })
    }
    async fn list_instances(&self) -> Result<InstanceResponse, ProviderError> {

        Ok(InstanceResponse {
            instances: vec![
                InstanceData{
                    name: "testing_instance".to_string(),
                    instance_id: "0000000000000000".to_string(),
                    state: "running".to_string(),
                    private_ip: "323.576.34.12".to_string(),
                },
            ],
        })
    }
    async fn list_parameters(&self, _path: Option<String>, _decrypt: bool) -> Result<ParameterResponse, ProviderError> {
        Ok(ParameterResponse {
            parameters: vec![
                ParameterData{
                    name: "String".to_string(),
                    r#type: "String".to_string(),
                    value: "testing".to_string(),
                },
            ],
        })
    }

    async fn list_container_registries(&self) -> Result<CregResponse<CregRepoResponse>, ProviderError> {
        Ok(CregResponse {
            response: vec![
                CregRepoResponse {
                    path: "registry1".to_string(),
                },
                CregRepoResponse {
                    path: "registry2".to_string(),
                },
            ],
        })
    }

    async fn list_container_registry_images(&self, _registry: String) -> Result<CregResponse<CregImageResponse>, ProviderError> {
        Ok(CregResponse{
            response: vec![
                CregImageResponse {
                    tag: "latest".to_string(),
                },
                CregImageResponse {
                    tag: "v1.0".to_string(),
                },
            ],
        })
    }
}
