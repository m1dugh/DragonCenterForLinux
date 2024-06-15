// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
  tauri::Builder::default()
      .setup(|app| {
          #[cfg(debug_assertions)] // n'incluez ce code que sur les versions de débogage

          {
              let window = app.get_window("main").unwrap();
              window.open_devtools();
              window.close_devtools();
          }
          Ok(())
      })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
