use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// Whether to run as daemon
    #[arg(long)]
    pub daemon: bool,

    /// Whether to redirect the output of the daemon in
    /// files.
    #[arg(long)]
    pub debug: bool,

    /// The command to send to the daemon
    #[arg(short, long)]
    pub command: Option<String>,
}
