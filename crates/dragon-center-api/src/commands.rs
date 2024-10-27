use crate::ec::{BatteryMode, CoolerBoost};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    ReadBattery,
    ReadShiftMode,
    ReadFanMode,
    ReadCoolerBoost,
    ReadAvailableShiftModes,
    ReadAvailableFanModes,
    WriteBattery(BatteryMode),
    WriteShiftMode(String),
    WriteFanMode(String),
    WriteCoolerBoost(CoolerBoost),
}

#[derive(Serialize, Deserialize)]
pub enum CommandResponse {
    Battery(BatteryMode),
    ShiftMode(String),
    AvailableShiftModes(Vec<String>),
    FanMode(String),
    AvailableFanModes(Vec<String>),
    CoolerBoost(CoolerBoost),
    Success,
    Error(String),
}
