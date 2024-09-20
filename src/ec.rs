use std::fs;
use std::io::{Error, ErrorKind, Read, Result, Write};


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

impl From<&str> for BatteryMode {
    fn from(val: &str) -> BatteryMode {
        match val {
            "min" => BatteryMode::Min,
            "medium" => BatteryMode::Medium,
            "max" | _ => BatteryMode::Max,
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

pub fn get_battery_mode() -> Result<BatteryMode> {

    let mut file = fs::OpenOptions::new()
        .read(true)
        .open("/sys/devices/platform/msi-ec/battery_mode")?;
    let mut buf = [0u8; 8];

    let _ = file.read(&mut buf)?;
    match std::str::from_utf8(&buf) {
        Ok(val) => Ok(BatteryMode::from(val)),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}
