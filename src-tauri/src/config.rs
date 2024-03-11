use crate::{
    model::{AppConfig, MacropadData},
    APP_HANDLE,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConfigFile<T> {
    data: T,
}

impl<T> ConfigFile<T>
where
    T: DeserializeOwned + Serialize,
{
    pub fn new(data: T) -> Self {
        ConfigFile { data }
    }
    fn parent_dir(file_name: &str) -> PathBuf {
        match APP_HANDLE.get().unwrap().path_resolver().app_data_dir() {
            Some(path) => {
                if !path.exists() {
                    let _ = fs::create_dir_all(path.to_string_lossy().to_string());
                }
                path.join(file_name)
            }
            None => panic!("appdir not found"),
        }
    }
    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn read_config(file_name: &str) -> io::Result<Self> {
        let file_path = ConfigFile::<PathBuf>::parent_dir(file_name);
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let data: T = serde_json::from_reader(reader)?;
        Ok(ConfigFile { data })
    }

    pub fn write_config(&self, file_name: &str) -> io::Result<()> {
        let file_path = ConfigFile::<PathBuf>::parent_dir(file_name);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        serde_json::to_writer_pretty(file, &self.data)?;

        Ok(())
    }
}

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

pub fn read_app_config() -> Result<AppConfig, std::io::Error> {
    let config_name = "app_config.json";
    match ConfigFile::<AppConfig>::read_config(&config_name) {
        Ok(result) => Ok(result.get_data().clone()),
        Err(_) => {
            let config = ConfigFile::new(AppConfig {
                click_throught: false,
                always_on_top: false,
            });
            config.write_config(&config_name)?;
            Ok(config.get_data().clone())
        }
    }
}
pub fn write_app_config(config: AppConfig) -> Result<(), std::io::Error> {
    let config_name = "app_config.json";
    let config_file = ConfigFile::<AppConfig>::new(config);
    config_file.write_config(config_name)?;
    Ok(())
}

pub fn read_macropad_config() -> Result<Vec<MacropadData>, std::io::Error> {
    let config_name = "config.json";
    match ConfigFile::<Vec<MacropadData>>::read_config(&config_name) {
        Ok(result) => Ok(result.get_data().clone()),
        Err(_) => {
            let config = ConfigFile::new(Vec::new());
            config.write_config(&config_name)?;
            Ok(config.get_data().clone())
        }
    }
}

// Function to read AppConfig from a JSON file
fn _read_app_config() -> Result<AppConfig, std::io::Error> {
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
                let _ = write_app_config(default_config.clone());
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
fn _write_app_config(config: &AppConfig) -> Result<(), std::io::Error> {
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
