// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::panic;

use dragon_center_lib::ec_write_available;

fn main() {

    if let Err(e) = ec_write_available() {
        panic!("{}", e);
    }

    dragon_center_gui_lib::run()
}
