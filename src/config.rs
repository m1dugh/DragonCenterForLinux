use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type Address = u64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataMap {
    pub realtime_data: Address,
    pub multi_data: Vec<Address>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FanRPMMap {
    pub address: Address,
    pub length: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedControllerMap {
    pub cpu_temp: DataMap,
    pub cpu_fan_speed: DataMap,
    pub gpu_temp: DataMap,
    pub gpu_fan_speed: DataMap,
    pub cooler_boost: Address,
    pub realtime_cpu_fan_rpm: FanRPMMap,
    pub realtime_gpu_fan_rpm: FanRPMMap,
    pub fan_mode: Address,
    pub usb_backlight: Address,
    pub battery_charging_threshold: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub file: String,
    pub current_config: String,
    pub configs: HashMap<String, Box<EmbeddedControllerMap>>,
}

impl fmt::Display for DataMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{realtime: {}, registers: ({:?})}}", self.realtime_data, self.multi_data)
    }
}

impl fmt::Display for FanRPMMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{address: {}, length: {}}}", self.address, self.length)
    }
}

impl fmt::Display for EmbeddedControllerMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}

pub fn read_config(filename: &str) -> Result<Box<Config>, &'static str>
{
    let file = match std::fs::File::open(filename) {
        Ok(val) => val,
        Err(_) => return Err("Could not open file"),
    };

    let object = match serde_yaml::from_reader(file) {
        Ok(val) => val,
        Err(_) => return Err("Could not parse yaml file"),
    };

    Ok(Box::new(object))
}
