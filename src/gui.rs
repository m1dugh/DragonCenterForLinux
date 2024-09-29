#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use crate::{client::Client, daemon::Config, ec::{BatteryMode, CoolerBoost}};
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

pub fn start_window() -> Result<(), Box<dyn Error>> {


    let config = Config::new()?;
    let mut client = Client::new(&config)?;

    let ui = AppWindow::new()?;

    let battery_mode = client.get_battery_mode()?;

    ui.set_battery_mode(battery_mode.to_string().into());
    ui.set_shift_current_value(client.get_current_shift_mode()?.into());
    ui.set_fan_current_value(client.get_current_fan_mode()?.into());

    let available_shift_modes: Vec<SharedString> = client.get_available_shift_modes()?
        .into_iter()
        .map(Into::into)
        .collect();
    ui.set_shift_model(ModelRc::new(VecModel::from(available_shift_modes.clone())));

    let available_fan_modes: Vec<SharedString> = client.get_available_fan_modes()?
        .into_iter()
        .map(Into::into)
        .collect();
    ui.set_fan_model(ModelRc::new(VecModel::from(available_fan_modes.clone())));

    ui.set_cooler_boost_value(client.get_cooler_boost()?.into());

    let mut moved_client = client.clone();
    ui.on_request_battery_update(move |battery| {
        let battery_mode = BatteryMode::from(battery.to_string().as_str());
        match moved_client.set_battery_mode(battery_mode) {
            Err(e) => eprintln!("{}", e),
            _ => {}
        }
    });

    let mut moved_client = client.clone();
    ui.on_request_shift_update(move |shift| match moved_client.set_current_shift_mode(shift.to_string()) {
        Err(e) => eprintln!("{}", e),
        _ => {}
    });

    let mut moved_client = client.clone();
    ui.on_request_fan_update(move |fan| match moved_client.set_current_fan_mode(fan.to_string()) {
        Err(e) => eprintln!("{}", e),
        _ => {}
    });

    let mut moved_client = client.clone();
    ui.on_request_cooler_boost_update({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let cooler_boost = CoolerBoost::from(ui.get_cooler_boost_value());
            match moved_client.set_cooler_boost(cooler_boost) {
                Err(e) => eprintln!("{}", e),
                _ => {}
            };
        }
    });

    ui.run()?;

    Ok(())
}
