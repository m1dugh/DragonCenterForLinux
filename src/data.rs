#[derive(Debug)]
pub struct Temperature {
    pub temperature: u8,
    pub cores: Vec<u8>,
}

#[derive(Debug)]
pub struct FanSpeed {
    pub speed: u8,
    pub fans: Vec<u8>,
}
