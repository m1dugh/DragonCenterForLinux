mod cli;
mod client;
mod config;
mod daemon;
mod data;
mod ec;

use crate::cli::Args;
use crate::client::run_command;
use crate::config::read_config;
use crate::ec::EmbeddedController;
use clap::Parser;

fn main() -> std::io::Result<()> {
    // let config = match read_config("config.yaml") {
    //     Ok(val) => val,
    //     Err(e) => panic!("{}", e),
    // };

    // let current_config = config.configs[&config.current_config].clone();

    // let _controller = EmbeddedController::new(current_config, &config.file)?;

    let args = Args::parse();

    match args.command {
        Some(command) => {
            return run_command(command);
        }
        None => {
            return daemon::run_daemon(&args);
        }
    }
}
