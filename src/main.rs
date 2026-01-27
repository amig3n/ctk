mod app;
use app::run_app;

mod actions;
mod cli;
mod providers;
mod outputs;

#[tokio::main]
async fn main() {
    match run_app().await {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
