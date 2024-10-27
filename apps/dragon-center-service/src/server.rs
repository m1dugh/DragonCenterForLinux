use std::fs::{self, Permissions};
use std::{path::Path, process::exit};
use std::os::unix::fs::{chown, PermissionsExt};

use dragon_center_api::{commands::{Command, CommandResponse}, ec};
use log::{debug, error, info, warn};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{UnixListener, UnixStream}, sync::mpsc::Receiver};

use crate::cli::Cli;

pub fn handle_command(command: Command) -> CommandResponse {
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


pub fn handle_client_request(mut stream: UnixStream, data: &[u8]) {
    let payload_str = String::from_utf8_lossy(data);
    let payload_str = payload_str.trim_matches(char::from(0));
    debug!("read payload: {}", payload_str);
    let command = match serde_json::from_str::<Command>(payload_str) {
        Ok(v) => v,
        Err(e) => {
            error!("Could not parse command: {}", e);
            return;
        }
    };

    let response = handle_command(command);
    let response = serde_json::to_string(&response)
        .expect("Could not serialize response (should never happen).");
    let _ = stream.write(response.as_bytes());
}

pub async fn server(config: Cli, mut shutdown_receiver: Receiver<()>) {
    env_logger::init();

    info!("Starting server listening on: {}", config.socket_path());

    let socket_path_buf = Path::new(&config.socket_path()).to_path_buf();
    let listener = UnixListener::bind(socket_path_buf.clone())
        .expect("Could not create socket");

    chown(
        config.socket_path(),
        config.uid,
        config.gid,
    ).expect("Could not own the socket");
    fs::set_permissions(config.socket_path(), Permissions::from_mode(0o770))
        .expect("Could not set permissions for socket");

    info!("Starting tokio server");
    tokio::spawn(async move{
        let signal = shutdown_receiver.recv().await;
        debug!("Receiving sigterm");
        match signal {
            Some(()) => {
                tokio::fs::remove_file(socket_path_buf)
                    .await
                    .expect("Failed to remove socket");
                exit(1);
            },
            None => {
                warn!("received nothing from the shutdown receiver.")
            },
        }
    });

    while let Ok((mut stream, _)) = listener.accept().await {
        let mut buffer: [u8; 1024] = [0u8; 1024];
        debug!("Accepting client");
        tokio::spawn(async move {
            let mut payload: Vec<u8> = Vec::new();
            loop {
                match stream.read(&mut buffer).await {
                    Ok(n) => {
                        if n == 0 {
                            handle_client_request(stream, payload.as_slice());
                            break;
                        }
                        let mut vec = buffer.into_iter().collect::<Vec<_>>();
                        payload.append(&mut vec);

                    },
                    Err(e) => {
                        error!("Received error while reading: {}", e);
                        break;
                    }
                };
            };
        });
    };
}
