// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use dragon_center_lib::{self, get_cooler_boost};

#[tauri::command]
fn greet(name: &str) -> String {
    match get_cooler_boost() {
        Err(e) => format!("Error: {e}"),
        Ok(v) => format!("Cooler boost mode {}", v.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
