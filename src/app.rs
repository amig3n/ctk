use crate::cli::{CLI, Commands, CloudProviders};
use clap::Parser;

use crate::actions::{ProviderActions, ProviderError};

use crate::providers::aws::AwsProvider;

use log::{info, debug, warn, error};

#[derive(Debug)]
pub enum AppError {
    ConnectionError,
    TimeoutError,
    PermissionError,
    GeneralError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ConnectionError => write!(f, "Connection error occurred"),
            AppError::TimeoutError => write!(f, "Operation timed out"),
            AppError::PermissionError => write!(f, "Permission denied"),
            AppError::GeneralError(msg) => write!(f, "General error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ProviderError> for AppError {
    fn from(error: ProviderError) -> Self {
        match error {
            ProviderError::ConfigurationError => AppError::GeneralError("Configuration error".to_string()),
            ProviderError::AuthenticationError => AppError::PermissionError,
            ProviderError::ResourceNotFound => AppError::GeneralError("Resource not found".to_string()),
            ProviderError::GeneralError(msg) => AppError::GeneralError(msg),
        }
    }
}


pub async fn run_app() -> Result<(), AppError> {
    debug!("Parsing command line arguments...");
    let cli = CLI::parse();

    let log_level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    // Initialize logger with the determined log level
    env_logger::Builder::from_env(
        env_logger::Env::default()
        .default_filter_or(log_level)
    ).init();

    info!("Log level set to: {}", log_level);
    debug!("CLI arguments: {:?}", cli);


    match cli.provider {

         CloudProviders::Aws => {
            debug!("Selected provider: AWS");
            let provider = AwsProvider::new();

            match cli.command {
                Commands::Whoami => {
                    debug!("Executing 'whoami' command for AWS provider");
                    let user_data = provider.who_am_i().await?;
                }

                _ => {
                    warn!("Command not exists or not-yet implemented");
                }
            }
         }

         _ => {
            error!("Selected provider is not supported yet.");
            return Err(AppError::GeneralError("Provider not supported".to_string()));
        }
    };

    debug!("Finished executing command.");
    // Application logic goes here
    Ok(())
}

