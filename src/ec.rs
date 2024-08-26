use crate::config::{Address, DataMap, EmbeddedControllerMap};
use crate::data::*;
use std::fs::{OpenOptions, File};
use std::io::{Result, Seek, Read, Write, SeekFrom};

static COOLER_BOOST_OFF: u8 = 0;
static COOLER_BOOST_ON: u8 = 0x80;

static FAN_DIVISOR: u32 = 478000;
static BATTERY_OFFSET: u8 = 0x80;

#[derive(Debug)]
pub enum FanMode {
    Advanced = 0x8c,
    Basic = 0x4c,
    Auto = 0x0c,
    Unknown = 0,
}

pub struct EmbeddedController {
    config: Box<EmbeddedControllerMap>,
    _file: File,
}

struct DataResult {
    realtime: u8,
    multi: Vec<u8>,
}

impl DataResult {
    fn to_temperature(self) -> Temperature {
        Temperature {
            temperature: self.realtime,
            cores: self.multi,
        }
    }

    fn to_fan_speed(self) -> FanSpeed {
        FanSpeed {
            speed: self.realtime,
            fans: self.multi,
        }
    }
}

impl EmbeddedController {
    pub fn new(config: Box<EmbeddedControllerMap>, filename: &str) -> std::io::Result<Self> {

        let f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(false)
            .open(filename)?;

        Ok(EmbeddedController {
            config,
            _file: f,
        })
    }

    fn read_byte(&mut self, address: Address) -> Result<u8>
    {
        let mut buffer = [0; 1];
        self._file.seek(SeekFrom::Start(address))?;
        self._file.read_exact(&mut buffer)?;

        Ok(buffer[0])
    }

    fn write_byte(&mut self, address: Address, value: u8) -> Result<()>
    {
        let mut buffer = [0; 1];
        buffer[0] = value;
        self._file.seek(SeekFrom::Start(address))?;

        self._file.write(&buffer)?;

        Ok(())
    }

    fn read_data(&mut self, config: &DataMap) -> Result<DataResult>
    {
        let realtime_address = config.realtime_data;
        let realtime =
            self.read_byte(realtime_address)?;

        let multi = config.multi_data.clone()
            .iter()
            .map(|address| self.read_byte(*address).unwrap())
            .collect();

        Ok(DataResult {
            realtime,
            multi,
        })
    }

    fn read_u16(&mut self, address: Address) -> Result<u16>
    {
        let mut buffer = [0; 2];

        self._file.seek(SeekFrom::Start(address))?;
        self._file.read_exact(&mut buffer)?;

        let upper: u16 = buffer[0].into();
        let lower: u16 = buffer[1].into();

        Ok(upper << 8 | lower)
    }

    pub fn read_cpu_temp(&mut self) -> Result<Temperature> {
        let config = self.config.cpu_temp.clone();

        Ok(self.read_data(&config)?.to_temperature())
    }

    pub fn read_gpu_temp(&mut self) -> Result<Temperature> {
        let config = self.config.gpu_temp.clone();
        Ok(self.read_data(&config)?.to_temperature())
    }

    pub fn read_cpu_fan_speed(&mut self) -> Result<FanSpeed> {
        let config = self.config.cpu_temp.clone();

        Ok(self.read_data(&config)?.to_fan_speed())
    }

    pub fn read_gpu_fan_speed(&mut self) -> Result<FanSpeed> {
        let config = self.config.gpu_temp.clone();

        Ok(self.read_data(&config)?.to_fan_speed())
    }

    pub fn read_gpu_fan_rpm(&mut self) -> Result<u32> {
        let config = self.config.realtime_gpu_fan_rpm.clone();
        let res = self.read_u16(config.address)?;

        Ok(match res.into() {
            0 => 0,
            val => FAN_DIVISOR / val,
        })
    }

    pub fn read_cpu_fan_rpm(&mut self) -> Result<u32> {
        let config = self.config.realtime_cpu_fan_rpm.clone();
        let res = self.read_u16(config.address)?;
        Ok(match res.into() {
            0 => 0,
            val => FAN_DIVISOR / val,
        })
    }

    pub fn read_cooler_boost(&mut self) -> Result<bool> {
        let address = self.config.cooler_boost;

        let value = self.read_byte(address)?;
        Ok(value == COOLER_BOOST_ON)
    }

    pub fn write_cooler_boost(&mut self, flag: bool) -> Result<()>
    {
        let address = self.config.cooler_boost;
        let value = match flag {
            true => COOLER_BOOST_ON,
            false => COOLER_BOOST_OFF,
        };

        self.write_byte(address, value)?;

        Ok(())
    }

    pub fn read_battery_threshold(&mut self) -> Result<u8> {
        let address = self.config.battery_charging_threshold;

        let res = self.read_byte(address)?;
        Ok(res - BATTERY_OFFSET)
    }

    pub fn write_battery_threshold(&mut self, threshold: u8) -> Result<()> {
        if threshold > 100 || threshold <= 30 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid value for threshold"));
        }

        let value = threshold + BATTERY_OFFSET;
        let address = self.config.battery_charging_threshold;
        self.write_byte(address, value)
    }

    pub fn read_fan_mode(&mut self) -> Result<FanMode> {
        let address = self.config.fan_mode;

        let res = self.read_byte(address)?;

        println!("{}", res);
        if res == FanMode::Auto as u8 {
            Ok(FanMode::Auto)
        } else if res == FanMode::Basic as u8 {
            Ok(FanMode::Basic)
        } else if res == FanMode::Advanced as u8 {
            Ok(FanMode::Advanced)
        } else {
            Ok(FanMode::Unknown)
        }
    }

    pub fn write_fan_mode(&mut self, mode: FanMode) -> Result<()> {
        let address = self.config.fan_mode;
        self.write_byte(address, mode as u8)
    }
}
