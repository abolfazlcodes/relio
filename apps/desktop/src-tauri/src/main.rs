#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    relio_desktop::desktop_runtime::run();
}
