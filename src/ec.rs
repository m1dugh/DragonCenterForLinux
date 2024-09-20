use std::fs;
use std::io::{Error, ErrorKind, Result, Write};


pub enum BatteryMode {
    Min,
    Medium,
    Max,
}

impl ToString for BatteryMode {
    fn to_string(&self) -> String {
        match self {
            BatteryMode::Max => "max",
            BatteryMode::Min => "min",
            BatteryMode::Medium => "medium",
        }.to_string()
    }
}

impl BatteryMode {
    pub fn from(val: &str) -> Result<BatteryMode> {
        match val {
            "min" => Ok(BatteryMode::Min),
            "medium" => Ok(BatteryMode::Medium),
            "max" => Ok(BatteryMode::Max),
            _ => Err(Error::new(ErrorKind::Other, "Could not find variant"))
        }
    }
}

pub fn set_battery_mode(battery_mode: BatteryMode) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("/sys/devices/platform/msi-ec/battery_mode")?;
    file.write_fmt(format_args!("{}", battery_mode.to_string()))?;
    Ok(())
}
