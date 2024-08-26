use daemonize::Daemonize;
use std::io::{Error, ErrorKind, Read};
use std::{
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

use crate::cli::Args;
use nix::unistd::Uid;

pub fn handle_client(mut stream: UnixStream) {
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
    let command = match command_builder.string() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("received: {}", command);
}

pub fn run_daemon(_args: &Args) -> std::io::Result<()> {
    if !Uid::effective().is_root() {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "The daemon can only be started as root",
        ));
    }

    if !_args.debug {
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

    // Delete socket in case it exists
    let _ = std::fs::remove_file("/run/dragon-center.sock");

    let listener = UnixListener::bind("/run/dragon-center.sock")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}
