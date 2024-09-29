use std::{io::{Error, ErrorKind, Read, Write}, ops::Deref, os::unix::net::UnixStream, rc::Rc};

use crate::{commands::{Command, CommandResponse}, daemon::Config, ec::{BatteryMode, CoolerBoost}};


#[derive(Clone)]
pub struct Client {
    config: Config,
}

fn read_response(stream: Rc<UnixStream>) -> Result<CommandResponse, Box<dyn std::error::Error>> {
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

    if command_str.is_empty() {
        let err = std::io::Error::new(std::io::ErrorKind::InvalidData, "Empty command");
        return Err(err.into());
    }

    Ok(serde_json::from_str::<CommandResponse>(command_str)?)
}

impl Client {
    pub fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Client {
            config: config.clone(),
        })
    }

    pub fn send_command(&mut self, command: Command) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        let mut stream = UnixStream::connect(self.config.socket_path.clone())?;

        let request = serde_json::to_string(&command)?;
        stream.write_all(request.as_bytes())?;

        let stream = Rc::new(stream);

        let response = read_response(stream.clone())?;
        Ok(response)
    }

    pub fn get_battery_mode(&mut self) -> Result<BatteryMode, Box<dyn std::error::Error>> {
        let response = self.send_command(Command::ReadBattery)?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::Battery(val) => Ok(val),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }

    pub fn set_battery_mode(&mut self, mode: BatteryMode) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.send_command(Command::WriteBattery(mode))?;
        match response {
            CommandResponse::Success => Ok(()),
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }

    pub fn get_available_shift_modes(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = self.send_command(Command::ReadAvailableShiftModes)?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::AvailableShiftModes(val) => Ok(val),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }

    pub fn get_available_fan_modes(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = self.send_command(Command::ReadAvailableFanModes)?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::AvailableFanModes(val) => Ok(val),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }


    pub fn get_current_shift_mode(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.send_command(Command::ReadShiftMode)?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::ShiftMode(val) => Ok(val),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }

    pub fn set_current_shift_mode(&mut self, mode: String) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.send_command(Command::WriteShiftMode(mode))?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::Success => Ok(()),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }

    pub fn get_current_fan_mode(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.send_command(Command::ReadFanMode)?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::ShiftMode(val) => Ok(val),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }

    pub fn set_current_fan_mode(&mut self, mode: String) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.send_command(Command::WriteFanMode(mode))?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::Success => Ok(()),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }

    pub fn get_cooler_boost(&mut self) -> Result<CoolerBoost, Box<dyn std::error::Error>> {
        let response = self.send_command(Command::ReadCoolerBoost)?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::CoolerBoost(val) => Ok(val),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }

    pub fn set_cooler_boost(&mut self, mode: CoolerBoost) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.send_command(Command::WriteCoolerBoost(mode))?;
        match response {
            CommandResponse::Error(e) => Err(Box::new(Error::new(ErrorKind::Other, e))),
            CommandResponse::Success => Ok(()),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Invalid response received"))),
        }
    }
}
