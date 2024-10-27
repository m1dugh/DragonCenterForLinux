use log::error;
use server::server;
use tokio::{signal, sync::mpsc::channel};
use crate::cli::Cli;
use clap::Parser;

pub (crate) mod server;
pub (crate) mod cli;

#[tokio::main]
async fn main() {

    let (shutdown_sender, shutdown_receiver) = channel::<()>(1);

    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                shutdown_sender.send(()).await.unwrap();
            },
            Err(e) => {
                error!("Error whilst stopping the server: {}", e);
            }
        };
    });

    let args: Cli = Cli::parse();
    server(args, shutdown_receiver).await;
}
