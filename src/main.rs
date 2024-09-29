// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.

mod ec;
mod tray;
mod daemon;
mod commands;
mod gui;
mod client;

use std::error::Error;

use daemon::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let program_name = std::env::args().take(1).next().unwrap_or("dragon-center".into());
    let program_name = program_name.split('/').filter(|p| p.len() > 0).last().unwrap();
    println!("Starting {}", program_name);
    match program_name {
        "dragon-center-daemon" => {
            let config = Config::new()?;
            daemon::start_daemon(&config)
        },
        "dragon-center-applet" => tray::start_tray(),
        "dragon-center" | _ => gui::start_window(),
    }
}
