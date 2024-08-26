mod ec;
mod config;
mod data;
mod cli;
mod daemon;
mod client;

use crate::ec::{EmbeddedController};
use crate::config::read_config;
use crate::client::run_command;
use crate::cli::Args;
use clap::Parser;


fn main() -> std::io::Result<()> {
    // let config = match read_config("config.yaml") {
    //     Ok(val) => val,
    //     Err(e) => panic!("{}", e),
    // };

    // let current_config = config.configs[&config.current_config].clone();

    // let _controller = EmbeddedController::new(current_config, &config.file)?;

    let args = Args::parse();

    if args.daemon {
        return daemon::run_daemon(&args);
    }

    match args.command {
        Some(command) => {
            return run_command(command);
        }
        None => {}
    }

    Ok(())
}
