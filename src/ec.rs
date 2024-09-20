use std::fs::{self, read_to_string};
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
        }
        .to_string()
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

pub enum CoolerBoost {
    On,
    Off,
}

impl From<&str> for CoolerBoost {
    fn from(val: &str) -> CoolerBoost {
        match val {
            "on" => CoolerBoost::On,
            "off" | _ => CoolerBoost::Off,
        }
    }
}

impl From<bool> for CoolerBoost {
    fn from(val: bool) -> CoolerBoost {
        match val {
            true => CoolerBoost::On,
            false => CoolerBoost::Off,
        }
    }
}

impl Into<bool> for CoolerBoost {
    fn into(self) -> bool {
        match self {
            Self::On => true,
            Self::Off => false,
        }
    }
}

impl ToString for CoolerBoost {
    fn to_string(&self) -> String {
        match self {
            Self::On => "on",
            Self::Off => "off",
        }
        .to_string()
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

pub fn get_available_shift_modes() -> Result<Vec<String>> {
    Ok(
        read_to_string("/sys/devices/platform/msi-ec/available_shift_modes")?
            .lines()
            .map(String::from)
            .collect(),
    )
}

pub fn get_current_shift_mode() -> Result<String> {
    match read_to_string("/sys/devices/platform/msi-ec/shift_mode")?
        .lines()
        .take(1)
        .map(String::from)
        .nth(0)
    {
        Some(val) => Ok(val),
        None => Err(Error::new(
            ErrorKind::Other,
            "Could not find the current shift mode",
        )),
    }
}

pub fn set_shift_mode(shift_mode: String) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("/sys/devices/platform/msi-ec/shift_mode")?;
    file.write_fmt(format_args!("{}", shift_mode))?;
    Ok(())
}

pub fn get_available_fan_modes() -> Result<Vec<String>> {
    Ok(
        read_to_string("/sys/devices/platform/msi-ec/available_fan_modes")?
            .lines()
            .map(String::from)
            .collect(),
    )
}

pub fn get_current_fan_mode() -> Result<String> {
    match read_to_string("/sys/devices/platform/msi-ec/fan_mode")?
        .lines()
        .take(1)
        .map(String::from)
        .nth(0)
    {
        Some(val) => Ok(val),
        None => Err(Error::new(
            ErrorKind::Other,
            "Could not find the current shift mode",
        )),
    }
}

pub fn set_fan_mode(fan_mode: String) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("/sys/devices/platform/msi-ec/fan_mode")?;
    file.write_fmt(format_args!("{}", fan_mode))?;
    Ok(())
}

pub fn get_cooler_boost() -> Result<CoolerBoost> {
    match read_to_string("/sys/devices/platform/msi-ec/cooler_boost")?
        .lines()
        .take(1)
        .map(CoolerBoost::from)
        .nth(0)
    {
        Some(val) => Ok(val),
        None => Err(Error::new(
            ErrorKind::Other,
            "Could not find the current shift mode",
        )),
    }
}

pub fn set_cooler_boost(cooler_boost: CoolerBoost) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("/sys/devices/platform/msi-ec/fan_mode")?;
    file.write_fmt(format_args!("{}", cooler_boost.to_string()))?;
    Ok(())
}
