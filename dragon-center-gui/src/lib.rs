// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use dragon_center_lib::battery::{get_battery_mode, set_battery_mode};

#[tauri::command]
fn get_battery_level() -> String {
    match get_battery_mode() {
        Err(e) => format!("Error: {e}"),
        Ok(v) => v.to_string(),
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
        .invoke_handler(tauri::generate_handler![get_battery_level, set_battery_level])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
