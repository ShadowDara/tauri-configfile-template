// for generating the config file

use serde::{Deserialize, Serialize};
use std::{fs::{self, File}, io::{Read}, path::{Path, PathBuf}};
use tauri::{AppHandle};
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub search_for_updates: bool,
    pub save_dates_automatically: bool,
    pub event_dates: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialUserSettings {
    pub search_for_updates: Option<bool>,
    pub save_dates_automatically: Option<bool>,
    pub event_dates: Option<Vec<String>>,
}

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle) -> Result<UserSettings, String> {
    match UserSettings::get_settings_path(&app)
        .and_then(|path| UserSettings::load_settings(&app, &path))
    {
        Ok(settings) => Ok(settings),
        Err(err) => Err(format!("Failed to load settings: {}", err)),
    }
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: UserSettings) -> Result<(), String> {
    let path = match UserSettings::get_settings_path(&app) {
        Ok(p) => p,
        Err(e) => return Err(format!("Path error: {}", e)),
    };

    settings.save(&path).map_err(|e| format!("Save error: {}", e))
}

pub fn init_settings(app: &AppHandle) -> UserSettings {
    let path = UserSettings::get_settings_path(app)
        .expect("Could not determine settings path");

    match UserSettings::load_settings(app, &path) {
        Ok(settings) => settings,
        Err(e) => {
            println!("Failed to load settings: {}", e);
            let settings = UserSettings::new();
            if let Err(e) = settings.save(&path) {
                println!("Failed to save default settings: {}", e);
            }
            settings
        }
    }
}

impl UserSettings {
    pub fn new() -> UserSettings {
        // standard settings!
        UserSettings {
            search_for_updates: false,
            save_dates_automatically: false,
            event_dates: vec!["2024-05-21".to_string()],
        }
    }

    pub fn get_settings_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let settings_dir = app
            .path()
            .config_dir()?
            .join("tauri-test-app");

        if !settings_dir.exists() {
            std::fs::create_dir_all(&settings_dir)?; // Erstelle alle fehlenden Verzeichnisse
        }

        let settings_file = settings_dir.join("settings.json");
        Ok(settings_file)
    }

    pub fn load_settings(_app: &AppHandle, path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            let config = Self::new();
            config.save(path)?; // <-- speichert unter dem angegebenen Pfad
            return Ok(config);
        }
    
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        let config: Result<UserSettings, _> = serde_json::from_str(&contents);
    
        if let Err(e) = &config {
            // nur wenn du `log` benutzt
            // error!("[conf::load] {}", e);
            println!("[conf::load] {}", e);
    
            let mut default_config = Self::new();
            default_config = default_config.amend(serde_json::from_str(&contents)?)?;
            default_config.save(path)?;
            return Ok(default_config);
        }
    
        Ok(config?)
    }

    pub fn amend(mut self, patch: PartialUserSettings) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(search_for_updates) = patch.search_for_updates {
            self.search_for_updates = search_for_updates;
        }
        if let Some(save_dates_automatically) = patch.save_dates_automatically {
            self.save_dates_automatically = save_dates_automatically;
        }
        if let Some(event_dates) = patch.event_dates {
            self.event_dates = event_dates;
        }
        Ok(self)
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
    
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(path, json)?;
        Ok(())
    }
}
