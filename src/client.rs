use std::{
    io::{Error, ErrorKind, Read, Write},
    os::unix::net::UnixStream,
};

use crate::ipc::{Command, CommandResponse};

fn send_command(client: &mut UnixStream, payload: &Command) -> std::io::Result<CommandResponse> {
    let payload = serde_json::to_string(&payload)?;

    client.write(payload.as_bytes())?;

    let mut response = Vec::<u8>::new();
    client.read_to_end(&mut response)?;

    let response = String::from_utf8(response).unwrap();
    let response = response.trim_matches(char::from(0));

    let response = serde_json::from_str(&response)?;

    Ok(response)
}

fn run_write_command(client: &mut UnixStream, fields: &Vec<&str>) -> std::io::Result<()> {
    if fields.len() != 3 {
        return Err(Error::new(
            ErrorKind::Other,
            "Usage: write <address> <value>",
        ));
    }
    let address = match fields[1].parse::<u16>() {
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        Ok(val) => val,
    };

    let value = match fields[2].parse::<u8>() {
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        Ok(val) => val,
    };

    let payload = Command::WriteCommand { address, value };

    match send_command(client, &payload)? {
        CommandResponse::Error(e) => Err(Error::new(ErrorKind::Other, e)),
        _ => Ok(()),
    }
}

fn run_battery_command(client: &mut UnixStream, fields: &Vec<&str>) -> std::io::Result<()> {
    let payload = match fields.len() {
        1 => Command::ReadBattery,
        2 => {
            let threshold = match fields[1].parse::<u8>() {
                Err(e) => return Err(Error::new(ErrorKind::Other, e)),
                Ok(val) => val,
            };

            Command::WriteBattery { threshold }
        }
        _ => return Err(Error::new(ErrorKind::Other, "Invalid syntax")),
    };

    match send_command(client, &payload)? {
        CommandResponse::Battery(val) => {
            println!("Battery level: {}", val);
            Ok(())
        }
        CommandResponse::Error(e) => Err(Error::new(ErrorKind::Other, e)),
        _ => Ok(()),
    }
}

pub fn run_command(command: String) -> std::io::Result<()> {
    let mut client = UnixStream::connect("/run/dragon-center.sock")?;

    let fields = command
        .split(' ')
        .filter(|el| el.len() > 0)
        .collect::<Vec<&str>>();
    if fields.len() < 1 {
        return Err(Error::new(ErrorKind::Other, "Invalid command"));
    }

    match fields[0] {
        "write" => run_write_command(&mut client, &fields),
        "battery" => run_battery_command(&mut client, &fields),
        _ => Err(Error::new(ErrorKind::Other, "Invalid command")),
    }?;

    Ok(())
}
