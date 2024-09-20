// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ec;

use std::error::Error;

use ec::BatteryMode;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let battery_mode = ec::get_battery_mode()?;

    ui.set_battery_mode(battery_mode.to_string().into());

    ui.on_request_battery_update(|battery| {
        let battery_mode = BatteryMode::from(battery.to_string().as_str());

        match ec::set_battery_mode(battery_mode) {
            Err(e) => eprintln!("{}", e),
            _ => {}
        }
    });

    ui.run()?;

    Ok(())
}
