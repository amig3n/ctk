mod app;
use app::run_app;

mod actions;
mod cli;
mod providers;


fn main() {
    match run_app() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
