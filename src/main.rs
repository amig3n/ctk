mod app;
use app::run_app;

mod actions;
mod cli;
mod providers;

use log::{info, error, debug, warn};

fn main() {
    //env_logger::init();
    //info!("CTK starting up...");
    
    match run_app() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
