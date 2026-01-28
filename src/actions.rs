// NOTE: will be refactored to use this trait later

#[allow(dead_code)]
/// Define possible errors that can occur in provider actions
pub enum ProviderError {
    ConfigurationError,
    ConnectionError,
    AuthenticationError,
    ResourceNotFound,
    TimeoutError,
    PermissionError,
    GeneralError(String),
}

#[allow(dead_code)]
/// Define list of actions that a single cloud provider should implement
pub trait ProviderActions {
    /// Identify the current user
    async fn who_am_i(&self) -> Result<String, ProviderError>;
    ///// Configure the provider
    //fn configure(&self) -> Result<(), ProviderError>;
    /// List avilable instances
    fn list_instances(&self) -> Result<Vec<String>, ProviderError>;
    /// List defined parameters
    fn list_parameters(&self) -> Result<Vec<String>, ProviderError>;
    ///// List container registtries
    //fn list_container_registries(&self) -> Result<Vec<String>, ProviderError>;
}
