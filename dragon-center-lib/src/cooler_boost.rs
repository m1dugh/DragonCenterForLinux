use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result, Write};
use std::fs::{self, read_to_string};

use crate::utils::sanitize_string;

#[derive(Serialize, Deserialize)]
pub enum CoolerBoost {
    On,
    Off,
}

impl From<&str> for CoolerBoost {
    fn from(val: &str) -> CoolerBoost {
        match sanitize_string(val) {
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
        .open("/sys/devices/platform/msi-ec/cooler_boost")?;
    file.write_fmt(format_args!("{}", cooler_boost.to_string()))?;
    Ok(())
}
