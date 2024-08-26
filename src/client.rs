use std::{io::Write, os::unix::net::UnixStream};


pub fn run_command(command: String) -> std::io::Result<()> {
    let mut client = UnixStream::connect("/run/dragon-center.sock")?;

    client.write(command.as_bytes())?;

    Ok(())
}
