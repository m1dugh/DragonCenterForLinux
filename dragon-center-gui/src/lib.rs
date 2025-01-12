// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use dragon_center_lib::battery::{get_battery_mode, set_battery_mode};
use dragon_center_lib::shift;
use dragon_center_lib::cooler_boost;

#[tauri::command]
fn get_cooler_boost() -> Option<bool> {
    match cooler_boost::get_cooler_boost() {
        Err(e) => {
            eprintln!("Error: {}", e);
            return None;
        },
        Ok(v) => Some(v.into()),
    }
}

#[tauri::command]
fn set_cooler_boost(boost: bool) -> bool {
    match cooler_boost::set_cooler_boost(boost.into()) {
        Err(e) => {
            eprintln!("Error: {}", e);
            false
        },
        Ok(()) => true
    }
}

#[tauri::command]
fn get_available_shift_modes() -> Option<Vec<String>> {
    match shift::get_available_shift_modes() {
        Err(e) => {
            eprintln!("Error: {}", e);
            return None;
        },
        Ok(v) => Some(v),
    }
}

#[tauri::command]
fn set_shift_mode(mode: String) -> bool {
    match shift::set_shift_mode(mode) {
        Err(e) => {
            eprintln!("Error: {}", e);
            false
        },
        Ok(()) => true
    }
}

#[tauri::command]
fn get_shift_mode() -> Option<String> {
    match shift::get_current_shift_mode() {
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        },
        Ok(v) => Some(v)
    }
}

#[tauri::command]
fn get_battery_level() -> Option<String> {
    match get_battery_mode() {
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        },
        Ok(v) => Some(v.to_string()),
    }
}

#[tauri::command]
fn set_battery_level(level: &str) -> bool {
    match set_battery_mode(level.into()) {
        Err(e) => {
            eprintln!("{}", e);
            false
        },
        Ok(()) => true,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_battery_level,
            set_battery_level,

            get_available_shift_modes,
            get_shift_mode,
            set_shift_mode,

            get_cooler_boost,
            set_cooler_boost,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
