use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "ctk")]
#[command(about = "CTK - Cloud Toolkit")]
#[command(author,version,about)]
pub struct CLI {
    // NOTE env value for this decorator is avilable to pass provider as ENV value - add it later
    #[arg(short, long, value_enum, default_value_t = CloudProviders::Aws)]
    pub provider: CloudProviders,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    // TODO: add global config for output

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum CloudProviders {
    /// Amazon Web Services
    Aws,
     
}

#[derive(Subcommand, Debug)]
pub enum Commands{
    ///// Configure CTK for selected provider
    //Config,
    ///// Show available providers
    //Providers,
    /// Show cloud instances
    Instances,
    /// Show parameters
    Params {
        /// Parameter path
        path: Option<String>,

        /// Flag to allow decryption of secure parameters
        #[arg(short, long, default_value_t = false)]
        decrypt: bool,
    },
    ///// Show cotainer registries
    //Creg, 
    /// Who am I?
    Whoami,
}


