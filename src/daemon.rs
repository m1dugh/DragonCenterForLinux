use daemonize::Daemonize;
use std::io::{ErrorKind, Read, Write};
use std::ops::Deref;
use std::rc::Rc;
use std::{env, fs, thread};
use std::fs::{remove_file, File, Permissions};
use std::os::unix::fs::{chown, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use log::{debug, error, info};

use crate::commands::{Command, CommandResponse};
use crate::ec;

#[derive(Clone)]
pub struct Config {
    pub socket_path: String,
    pub uid: u32,
    pub gid: u32,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub nofork: bool,
}

fn find_var(suffix: &str) -> Option<String> {
    let name: String = format!("DRAGON_CENTER_{}", suffix);

    return env::vars().
        find(|(key, _)| *key == name).
        map(|(_, value)| value)
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let uid = find_var("UID").unwrap_or("0".into()).parse::<u32>()?;
        let gid = find_var("GID").unwrap_or("0".into()).parse::<u32>()?;

        Ok(Config {
            socket_path: find_var("SOCKET_PATH").
                unwrap_or("/run/dragon-center.sock".into()),
            uid,
            gid,
            stdout: find_var("STDOUT"),
            stderr: find_var("STDERR"),
            nofork: find_var("NO_FORK").is_some(),
        })
    }
}

fn read_request(stream: Rc<UnixStream>) -> Result<Command, Box<dyn std::error::Error>> {
    let mut stream = stream.deref();
    let mut command_builder = string_builder::Builder::new(1024);
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        let size = stream.read(&mut buf)?;
        command_builder.append(&buf[..]);
        if size < buf.len() {
            break;
        }
    };

    let command_str = command_builder.string()?;

    let command_str = command_str.trim_matches(char::from(0));

    debug!("Received command '{}'", command_str);
    if command_str.is_empty() {
        let err = std::io::Error::new(std::io::ErrorKind::InvalidData, "Empty command");
        return Err(err.into());
    }

    Ok(serde_json::from_str::<Command>(command_str)?)
}

fn handle_command(command: Command) -> CommandResponse {
    match command {
        Command::ReadBattery => match ec::get_battery_mode() {
            Ok(mode) => CommandResponse::Battery(mode),
            Err(e) => CommandResponse::Error(e.to_string()),
        },
        Command::ReadShiftMode => match ec::get_current_shift_mode() {
            Ok(mode) => CommandResponse::ShiftMode(mode),
            Err(e) => CommandResponse::Error(e.to_string()),
        },
        Command::ReadFanMode => match ec::get_current_fan_mode() {
            Ok(mode) => CommandResponse::FanMode(mode),
            Err(e) => CommandResponse::Error(e.to_string()),
        },
        Command::ReadCoolerBoost => match ec::get_cooler_boost() {
            Ok(mode) => CommandResponse::CoolerBoost(mode),
            Err(e) => CommandResponse::Error(e.to_string()),
        },
        Command::ReadAvailableShiftModes => match ec::get_available_shift_modes() {
            Ok(mode) => CommandResponse::AvailableShiftModes(mode),
            Err(e) => CommandResponse::Error(e.to_string()),
        },
        Command::ReadAvailableFanModes => match ec::get_available_fan_modes() {
            Ok(mode) => CommandResponse::AvailableFanModes(mode),
            Err(e) => CommandResponse::Error(e.to_string()),
        },

        Command::WriteBattery(mode) => match ec::set_battery_mode(mode) {
            Ok(_) => CommandResponse::Success,
            Err(e) => CommandResponse::Error(e.to_string()),
        },
        Command::WriteShiftMode(mode) => match ec::set_shift_mode(mode) {
            Ok(_) => CommandResponse::Success,
            Err(e) => CommandResponse::Error(e.to_string()),
        },
        Command::WriteFanMode(mode) => match ec::set_fan_mode(mode) {
            Ok(_) => CommandResponse::Success,
            Err(e) => CommandResponse::Error(e.to_string()),
        },
        Command::WriteCoolerBoost(mode) => match ec::set_cooler_boost(mode) {
            Ok(_) => CommandResponse::Success,
            Err(e) => CommandResponse::Error(e.to_string()),
        },
    }
}

fn send_response(stream: Rc<UnixStream>, response: CommandResponse) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = stream.deref();
    let response = serde_json::to_string(&response)?;
    debug!("Sending response '{}'", response);
    let _ = stream.write(response.as_bytes())?;
    Ok(())
}

fn handle_client(stream: UnixStream) {

    let stream = Rc::new(stream);

    let command = match read_request(stream.clone()) {
        Err(e) => {
            error!("{}", e);
            return;
        },
        Ok(val) => val,
    };

    let response = handle_command(command);

    if let Err(e) = send_response(stream.clone(), response) {
        error!("{}", e);
        return;
    }
}

fn start_listener(config: &Config) -> Result<(), Box<dyn std::error::Error>> {

    if let Err(e) = remove_file(config.socket_path.clone()) {
        if e.kind() != ErrorKind::NotFound {
            return Err(Box::new(e));
        }
    }
    let listener = UnixListener::bind(config.socket_path.clone())?;

    chown(config.socket_path.clone(), Some(config.uid), Some(config.gid))?;
    fs::set_permissions(config.socket_path.clone(), Permissions::from_mode(0o770))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            },
            Err(err) => {
                error!("Error: {}", err);
            }
        }
    }


    Ok(())
}

pub fn start_daemon(config: &Config) -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();
    if ! config.nofork {

        let mut daemonize = Daemonize::new()
            .pid_file("/tmp/dragon-center.pid");

        if let Some(filename) = &config.stdout {
            let file = File::create(filename)?; 
            daemonize = daemonize.stdout(file);
        }

        if let Some(filename) = &config.stderr {
            let file = File::create(filename)?; 
            daemonize = daemonize.stderr(file);
        }

        info!("Starting daemon");
        daemonize.start()?;
        info!("Starting daemonized instance");
    } else {
        info!("Starting undaemonized instance");
    }


    start_listener(&config)?;

    Ok(())
}
