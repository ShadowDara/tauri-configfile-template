// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;

use settings::{
    get_settings,
    save_settings,
    init_settings
};

fn main() {
    env_logger::init();
    if let Err(e) = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings
        ])
        .setup(|_app| {
            // for loading settings for the backend and creating new settings automatically on startup when are not avalable anymore!
            let settings = init_settings(&_app.handle());
            println!("settings loaded: {:?}", settings);
            Ok(())
        })
        .run(tauri::generate_context!())
    {
        eprintln!("Error while starting the app: {:?}", e);
        std::process::exit(1);
    }
}
