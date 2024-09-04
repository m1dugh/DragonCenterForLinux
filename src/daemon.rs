use daemonize::Daemonize;
use std::io::{Error, ErrorKind, Read, Write};
use std::{
    os::unix::net::{UnixListener, UnixStream},
    thread,
};
use string_builder::ToBytes;

use crate::cli::Args;
use crate::config::read_config;
use crate::ec::EmbeddedController;
use crate::ipc::Command::{self, ReadBattery, ReadCommand, WriteBattery, WriteCommand};
use crate::ipc::CommandResponse;
use nix::unistd::Uid;

pub fn handle_client(ec: Box<&mut EmbeddedController>, mut stream: UnixStream) {
    let mut command_builder = string_builder::Builder::new(1024);
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
            Ok(size) => {
                command_builder.append(&buf[..]);
                if size < buf.len() {
                    break;
                }
            }
        }
    }

    let command_str = match command_builder.string() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let command_str = command_str.trim_matches(char::from(0));

    if command_str.is_empty() {
        return;
    }

    let command = match serde_json::from_str::<Command>(command_str) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    match command {
        WriteCommand { address, value } => {
            println!("Setting {} at {}", value, address);
        }
        ReadCommand { address } => {
            println!("Reading {}", address);
        }
        ReadBattery => {
            let response = match ec.read_battery_threshold() {
                Ok(val) => CommandResponse::Battery(val),
                Err(e) => CommandResponse::Error(e.to_string()),
            };

            let response = serde_json::to_string(&response).unwrap();
            let _ = stream.write(response.as_bytes());
        }
        WriteBattery { threshold } => {
            let response = match ec.write_battery_threshold(threshold) {
                Ok(()) => CommandResponse::Success,
                Err(e) => CommandResponse::Error(e.to_string()),
            };

            let response = serde_json::to_string(&response).unwrap();
            let _ = stream.write(response.as_bytes());
        }
    }
}

pub fn run_daemon(args: &Args) -> std::io::Result<()> {
    if !Uid::effective().is_root() {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "The daemon can only be started as root",
        ));
    }

    if args.daemon {
        let daemonize = Daemonize::new().pid_file("/tmp/dragon-center.pid");

        match daemonize.start() {
            Ok(_) => println!("Starting daemon"),
            Err(e) => {
                eprintln!("error: {}", e);
                return Err(Error::new(ErrorKind::Other, "Could not start daemon"));
            }
        }
    } else {
        println!("Starting undaemonized instance");
    }

    let config_file = args.config.clone().unwrap_or("config.yaml".to_string());

    let config = match read_config(config_file.as_str()) {
        Ok(val) => val,
        Err(e) => panic!("{}", e),
    };

    let current_config = config.configs[&config.current_config].clone();

    // Delete socket in case it exists
    let _ = std::fs::remove_file("/run/dragon-center.sock");

    let listener = UnixListener::bind("/run/dragon-center.sock")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut controller = EmbeddedController::new(current_config.clone(), &config.file)?;
                thread::spawn(move || handle_client(Box::new(&mut controller), stream));
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}
