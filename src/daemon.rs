use std::{os::unix::net::{UnixListener, UnixStream}, thread};
use daemonize::Daemonize;
use std::io::{ErrorKind, Error, Read};

use crate::cli::Args;
use nix::unistd::Uid;

pub fn handle_client(mut stream: UnixStream) {
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
            Ok(size) => {
                if size < buf.len() {
                    break
                }
                println!("result: {}", String::from_utf8_lossy(&buf));
            }
        }
    }
}

pub fn run_daemon(_args: &Args) -> std::io::Result<()> {

    if !Uid::effective().is_root() {
        return Err(Error::new(ErrorKind::PermissionDenied, "The daemon can only be started as root"))
    }

    let daemonize = Daemonize::new()
        .pid_file("/tmp/dragon-center.pid");

    match daemonize.start() {
        Ok(_) => println!("Starting daemon"),
        Err(e) => {
            eprintln!("error: {}", e);
            return Err(Error::new(ErrorKind::Other, "Could not start daemon"))
        }
    }

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
