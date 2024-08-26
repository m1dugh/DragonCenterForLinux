mod ec;
mod config;
mod data;
mod cli;
mod daemon;

use crate::ec::{EmbeddedController};
use crate::config::read_config;
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

    Ok(())
}
