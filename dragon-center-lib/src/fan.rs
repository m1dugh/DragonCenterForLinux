use std::fs::{self, read_to_string};
use std::io::{Error, ErrorKind, Result, Write};

use crate::utils::sanitize_string;

pub fn get_available_fan_modes() -> Result<Vec<String>> {
    Ok(
        read_to_string("/sys/devices/platform/msi-ec/available_fan_modes")?
            .lines()
            .map(|s| String::from(sanitize_string(s)))
            .collect(),
    )
}

pub fn get_current_fan_mode() -> Result<String> {
    match read_to_string("/sys/devices/platform/msi-ec/fan_mode")?
        .lines()
        .take(1)
        .map(|s| String::from(sanitize_string(s)))
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
