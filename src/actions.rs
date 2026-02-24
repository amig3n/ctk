use async_trait::async_trait;
use crate::responses::*;
// NOTE: will be refactored to use this trait later

/// Define possible errors that can occur in provider actions
pub enum ProviderError {
    ConfigurationError,
    ConnectionError,
    AuthenticationError,
    ResourceNotFound,
    TimeoutError,
    PermissionError,
    GeneralError(String),
    EmptyResponse(String),
}

/// Define list of actions that a single cloud provider should implement
#[async_trait]
pub trait ProviderActions {
    /// Identify the current user
    async fn who_am_i(&self) -> Result<UserResponse, ProviderError>;
    ///// Configure the provider
    //fn configure(&self) -> Result<(), ProviderError>;
    /// List available instances
    async fn list_instances(&self) -> Result<InstanceResponse, ProviderError>;
    /// List defined parameters
    async fn list_parameters(&self, path: Option<String>, decrypt: bool) -> Result<ParameterResponse, ProviderError>;
    /// List container registtries
    async fn list_container_registries(&self) -> Result<CregResponse<CregRepoResponse>, ProviderError>;
    /// List container registry images
    async fn list_container_registry_images(&self, registry: String) -> Result<CregResponse<CregImageResponse>, ProviderError>;


}
