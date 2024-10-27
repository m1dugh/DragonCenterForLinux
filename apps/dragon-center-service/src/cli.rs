use clap::Parser;

/// The dragon center for linux daemon
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Cli {
    /// The path of the domain socket
    /// Defaults to /var/run/dragon-center.sock
    #[arg(short = 's', long)]
    socket_path: Option<String>,

    /// The uid of the owner of the socket
    #[arg(long)]
    pub uid: Option<u32>,

    /// The gid of the owner of the socket
    #[arg(long)]
    pub gid: Option<u32>,
}

impl Cli {
    pub fn socket_path(&self) -> String {
        self.socket_path.clone().unwrap_or("/var/run/dragon-center.sock".to_string())
    }
}
