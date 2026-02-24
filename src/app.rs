use crate::cli::{CLI, Commands, CloudProviders};
use clap::Parser;
use crate::actions::{ProviderActions, ProviderError};
use log::{info, debug, warn, error};

use crate::providers::aws::AwsProvider;
use crate::providers::cloudflare::CloudflareProvider;

use crate::outputs::table::{Table, TableColumnFormat, TableError};
use crate::responses::*;

#[derive(Debug)]
pub enum AppError {
    AuthenticationError(String),
    ConnectionError,
    TimeoutError,
    PermissionError,
    GeneralError(String),
    OutputTableError(TableError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::AuthenticationError(err) => write!(f, "Authentication error occurred: {}", err),
            AppError::ConnectionError => write!(f, "Connection error occurred"),
            AppError::TimeoutError => write!(f, "Operation timed out"),
            AppError::PermissionError => write!(f, "Permission denied"),
            AppError::GeneralError(msg) => write!(f, "General error: {}", msg),
            AppError::OutputTableError(msg) => write! (f, "Output error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ProviderError> for AppError {
    fn from(error: ProviderError) -> Self {
        match error {
            ProviderError::ConfigurationError => AppError::GeneralError("Configuration error".to_string()),
            ProviderError::AuthenticationError => AppError::AuthenticationError("Authentication failed".to_string()),
            ProviderError::ResourceNotFound => AppError::GeneralError("Resource not found".to_string()),
            ProviderError::GeneralError(msg) => AppError::GeneralError(msg),
            ProviderError::TimeoutError => AppError::TimeoutError,
            ProviderError::ConnectionError => AppError::ConnectionError,
            ProviderError::PermissionError => AppError::PermissionError,
            ProviderError::EmptyResponse(msg) => AppError::GeneralError(format!("Empty response: {}", msg)),
        }
    }
}

impl From<TableError> for AppError {
    fn from(error: TableError) -> Self {
        AppError::OutputTableError(error)
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
    
    let provider: Box<dyn ProviderActions> = match cli.provider {
        CloudProviders::Aws => {
            debug!("Using AWS provider");
            Box::new(AwsProvider::new())
        },
        CloudProviders::CF => {
            debug!("Using Cloudflare Provider");
            Box::new(CloudflareProvider::new())
        },
        _ => {
            return Err(AppError::GeneralError("Provider not implemented".to_string()))
        }
    };

    match cli.command {
        Commands::Whoami => {
            debug!("Executing 'whoami' for {} provider", cli.provider.to_string());
            let user_data: UserResponse = provider.who_am_i().await?;
            let table: Table = user_data.into();
            table.with_padding(2).render()?;
        }

        Commands::Instances => {
            debug!("Executing 'instances' for {} provider", cli.provider.to_string());
            let instances: InstanceResponse = provider.list_instances().await?;
            let table: Table = instances.into();
            table.with_padding(2).render()?;

        }

        Commands::Params {path, decrypt} => {
            debug!("Executing 'params' for {} provider", cli.provider.to_string());
            let params: ParameterResponse = provider.list_parameters(path, decrypt).await?;
            let table: Table = params.into();
            table.with_padding(2).render()?;
        }

        Commands::Creg { path } => {
            debug!("Executing 'creg' for {} provider", cli.provider.to_string());
            match path {
                Some(registry) => {
                    debug!("Obtaining images details");
                    let images: CregResponse<CregImageResponse> = provider.list_container_registry_images(registry).await?;
                    let mut table: Table = Table::new(vec!["Tag"])
                        .with_padding(2);

                    images.response.iter()
                        .map(|image| {
                            table.push(vec![
                                image.tag.clone(),
                            ]);
                        }).collect::<Vec<_>>();

                    table.render()?;
                },
                None => {
                    debug!("Obtaining container registry list");
                    let registries: CregResponse<CregRepoResponse> = provider.list_container_registries().await?;
                    let mut table: Table = Table::new(vec!["ECR Name"])
                        .with_padding(2);

                    registries.response.iter()
                        .map(|creg| {
                            table.push(vec![creg.to_string()]);
                        }).collect::<Vec<_>>();
                    table.render()?;
                },
            }
        }
    }
 

    //TODO move all display logic outside match

    debug!("Finished executing command.");
    Ok(())
}

