// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ec;

use std::error::Error;

use ec::{BatteryMode, CoolerBoost};
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let battery_mode = ec::get_battery_mode()?;

    ui.set_battery_mode(battery_mode.to_string().into());
    ui.set_shift_current_value(ec::get_current_shift_mode()?.into());
    ui.set_fan_current_value(ec::get_current_fan_mode()?.into());

    let available_shift_modes: Vec<SharedString> = ec::get_available_shift_modes()?
        .into_iter()
        .map(Into::into)
        .collect();
    ui.set_shift_model(ModelRc::new(VecModel::from(available_shift_modes.clone())));

    let available_fan_modes: Vec<SharedString> = ec::get_available_fan_modes()?
        .into_iter()
        .map(Into::into)
        .collect();
    ui.set_fan_model(ModelRc::new(VecModel::from(available_fan_modes.clone())));

    ui.set_cooler_boost_value(ec::get_cooler_boost()?.into());

    ui.on_request_battery_update(|battery| {
        let battery_mode = BatteryMode::from(battery.to_string().as_str());

        match ec::set_battery_mode(battery_mode) {
            Err(e) => eprintln!("{}", e),
            _ => {}
        }
    });

    ui.on_request_shift_update(|shift| match ec::set_shift_mode(shift.to_string()) {
        Err(e) => eprintln!("{}", e),
        _ => {}
    });

    ui.on_request_fan_update(|fan| match ec::set_fan_mode(fan.to_string()) {
        Err(e) => eprintln!("{}", e),
        _ => {}
    });

    ui.on_request_cooler_boost_update({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let cooler_boost = CoolerBoost::from(ui.get_cooler_boost_value());
            match ec::set_cooler_boost(cooler_boost) {
                Err(e) => eprintln!("{}", e),
                _ => {}
            };
        }
    });

    ui.run()?;

    Ok(())
}
