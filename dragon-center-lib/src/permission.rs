use std::path::Path;
use nix::unistd::Uid;

pub fn ec_write_available() -> Result<(), String> {

    if !Uid::effective().is_root() {
        return Err("Program should be run as root".to_string());
    }

    let path = Path::new("/sys/devices/platform/msi-ec/");
    if !path.exists() || !path.is_dir() {
        return Err("Expected folder '/sys/devices/platform/msi-ec/' to exist".to_string());
    }

    Ok(())
}

