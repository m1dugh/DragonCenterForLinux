mod ec;
mod config;
mod data;

use crate::ec::{FanMode, EmbeddedController};
use crate::config::read_config;
use std::sync::Mutex;

use tauri::Manager;

#[tauri::command]
fn get_battery_threshold(state: tauri::State<Mutex<EmbeddedController>>) -> Result<u8, String> {
    let mut controller = state.lock().unwrap();
    let battery = match controller.read_battery_threshold() {
        Ok(val) => val,
        Err(_) => return Err("error reading battery".into())
    };
    Ok(battery)
}

#[tauri::command]
fn set_battery_threshold(threshold: u8, state: tauri::State<Mutex<EmbeddedController>>) -> Result<(), String> {
    let mut controller = state.lock().unwrap();
    match controller.write_battery_threshold(threshold) {
        Err(_) => return Err("error reading battery".into()),
        _ => Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let config = match read_config("config.yaml") {
        Ok(val) => val,
        Err(e) => panic!("{}", e),
    };

    let current_config = config.configs[&config.current_config].clone();

    let controller = EmbeddedController::new(current_config, &config.file)?;

  tauri::Builder::default()
      /* .setup(|app| {
          #[cfg(debug_assertions)] // n'incluez ce code que sur les versions de débogage

          {
              let window = app.get_window("main").unwrap();
              window.open_devtools();
              window.close_devtools();
          }
          Ok(())
      }) */
      .manage(Mutex::new(controller))
    .invoke_handler(tauri::generate_handler![get_battery_threshold, set_battery_threshold])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}
