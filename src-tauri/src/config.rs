use tauri::App;

use crate::{
    model::{AppConfig, MacropadData},
    APP_HANDLE,
};
use std::{
    fs::{self, File},
    io::Write,
};
pub fn read_action_lists() -> Vec<String> {
    let mut config_data: Vec<String> = Vec::new();
    match APP_HANDLE.get().unwrap().path_resolver().app_data_dir() {
        Some(path) => {
            if !path.exists() {
                let _ = fs::create_dir_all(path.to_string_lossy().to_string());
            }

            let config_path = path.join("action_lists.json");

            if fs::metadata(&config_path).is_ok() {
                match fs::read_to_string(&config_path) {
                    Ok(result) => match serde_json::from_str::<Vec<String>>(&result) {
                        Ok(data_result) => config_data = data_result,
                        Err(_) => println!("error parsing reading file"),
                    },
                    Err(_) => println!("error reading file"),
                }
            } else {
                //write default file
                let mut file = File::create(&config_path.to_string_lossy().to_string()).unwrap();
                let serialized = serde_json::to_string_pretty(&config_data).unwrap();
                file.write_all(serialized.as_bytes()).unwrap();
            }
        }
        None => {
            println!("config not found!");
        }
    }
    config_data
}
pub fn read_config() -> Vec<MacropadData> {
    let mut config_data: Vec<MacropadData> = Vec::new();
    match APP_HANDLE.get().unwrap().path_resolver().app_data_dir() {
        Some(path) => {
            if !path.exists() {
                let _ = fs::create_dir_all(path.to_string_lossy().to_string());
            }

            let config_path = path.join("config.json");

            if fs::metadata(&config_path).is_ok() {
                match fs::read_to_string(&config_path) {
                    Ok(result) => match serde_json::from_str::<Vec<MacropadData>>(&result) {
                        Ok(data_result) => config_data = data_result,
                        Err(err) => println!("error parsing reading file,{}", err),
                    },
                    Err(_) => println!("error reading file"),
                }
            } else {
                //write default file
                let mut file = File::create(&config_path.to_string_lossy().to_string()).unwrap();
                let serialized = serde_json::to_string_pretty(&config_data).unwrap();
                file.write_all(serialized.as_bytes()).unwrap();
            }
        }
        None => {
            println!("config not found!");
        }
    }
    config_data
}

// Function to read AppConfig from a JSON file
pub fn read_app_config() -> Result<AppConfig, std::io::Error> {
    match APP_HANDLE.get().unwrap().path_resolver().app_data_dir() {
        Some(path) => {
            if !path.exists() {
                let _ = fs::create_dir_all(path.to_string_lossy().to_string());
            }

            let config_path = path.join("app_config.json");
            if fs::metadata(&config_path).is_ok() {
                let json_content = fs::read_to_string(config_path)?;
                let config: AppConfig = serde_json::from_str(&json_content)?;

                Ok(config)
            } else {
                let default_config = AppConfig {
                    click_throught: false,
                    always_on_top: false,
                };
                let _ = write_app_config(&default_config);
                Ok(default_config)
            }
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get app data directory",
        )),
    }
}

// Function to write AppConfig to a JSON file
pub fn write_app_config(config: &AppConfig) -> Result<(), std::io::Error> {
    match APP_HANDLE.get().unwrap().path_resolver().app_data_dir() {
        Some(path) => {
            if !path.exists() {
                let _ = fs::create_dir_all(path.to_string_lossy().to_string());
            }

            let config_path = path.join("app_config.json");
            let json_content = serde_json::to_string_pretty(config)?;
            fs::write(config_path, json_content)?;
        }
        None => println!("not found"),
    }

    Ok(())
}
