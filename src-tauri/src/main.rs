#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::{info, trace, warn};
use std::iter::Map;

use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use tauri::Manager;

#[tauri::command]
fn fetch_options() -> Vec<String> {
    let options = vec![
        "qiyezhuce_7145_02".to_string(),
        "wy".to_string(),
        "pepsi_cn_test".to_string(),
    ];
    options
}

#[tauri::command]
fn kick_back(select: &str) {
    log::info!("select: {}", select);
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    match select {
        "qiyezhuce_7145_02" => {
            // wait for 1 second
            std::thread::sleep(std::time::Duration::from_secs(2));
            // send keybord event using enigo
            // clear the input field by sent 100 backspace key
            enigo.text("qiyezhuce_7145_02");
            enigo.key(Key::Tab, Click);

            enigo.text("ysf");
            enigo.key(Key::Tab, Click);

            enigo.text("a888888");
            enigo.key(Key::Return, Click);
        }
        "wy" => {
            println!("wy");
        }
        "pepsi_cn_test" => {
            println!("pepsi_cn_test");
        }
        _ => {
            println!("default");
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_options, kick_back])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
