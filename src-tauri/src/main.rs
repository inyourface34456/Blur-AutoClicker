#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// to build `npm exec tauri build -- --no-bundle --ci`

use app_lib::run;

fn main() {
    run();
}
