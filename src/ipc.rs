use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CommandResponse {
    Success,
    Error(String),
    Battery(u8),
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    WriteCommand { address: u16, value: u8 },
    ReadCommand { address: u8 },
    ReadBattery,
    WriteBattery { threshold: u8 },
}
