mod config;
mod ec;
mod data;

use crate::ec::EmbeddedController;

use crate::config::read_config;

fn main() -> std::io::Result<()> {
    let config = match read_config("config.yaml") {
        Ok(config) => config,
        Err(e) => panic!("{}", e)
    };

    let current_config = config.configs[&config.current_config].clone();
    let mut controller = EmbeddedController::new(current_config, &config.file)?;

    let cpu_temp = controller.read_cpu_temp()?;
    println!("cpu {:?}", cpu_temp);

    let cpu_fan = controller.read_cpu_fan_rpm()?;
    println!("cpu fan {:?}", cpu_fan);

    let gpu_fan = controller.read_gpu_fan_rpm()?;
    println!("gpu fan {:?}", gpu_fan);

    let boost = controller.read_cooler_boost()?;
    println!("cooler boost {}", boost);

    let battery = controller.read_battery_threshold()?;
    println!("battery {}", battery);

    let mode = controller.read_fan_mode()?;
    println!("fan mode {:?}", mode);

    Ok(())
}
