use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// Whether to run as daemon
    #[arg(long)]
    pub daemon: bool
}
