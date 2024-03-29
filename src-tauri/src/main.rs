// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use config::read_action_lists;
use config::read_app_config;
use config::read_config;
use global_hotkey::GlobalHotKeyEvent;
use macropad::MacropPad;
use model::AppConfig;
use model::MacropadData;
use std::sync::OnceLock;
use std::time::Duration;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTrayMenu, SystemTrayMenuItem};
use window_vibrancy::{apply_acrylic, apply_mica};

use crate::config::write_app_config;

mod config;
mod macropad;
mod model;

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn action_lists() -> Vec<String> {
    read_action_lists()
}
#[tauri::command]
fn filter_keys(key: &str) -> Vec<MacropadData> {
    let alldata = read_config();
    alldata
        .into_iter()
        .filter(|data| data.key_1 == Some(key.to_string()))
        .filter(|data| data.key_2 != Some("".into()))
        .collect()
}

fn setting_up_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let settings = CustomMenuItem::new("config".to_string(), "Config");
    let clicktru = CustomMenuItem::new("clicktru".to_string(), "Click through");

    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_item(clicktru)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn main() {
    let system_tray = setting_up_tray();
    let mut macropad = MacropPad::new().expect("failed");
    let _hotkey_manager = &macropad.register();
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "windows")]
            apply_mica(&window, None)
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            APP_HANDLE.set(app.app_handle()).unwrap();

            tauri::async_runtime::spawn(async move {
                loop {
                    if let Ok(event) = global_hotkey_channel.try_recv() {
                        let _ = &macropad.process_key(&event);
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
            });
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {}
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "clicktru" => match app.get_window("main") {
                    Some(window) => {
                        let _ = window.set_always_on_top(true);
                        let mut config = read_app_config().unwrap();
                        config.click_throught = !config.click_throught;
                        let _ = write_app_config(config.clone());
                        let item_handle = app.tray_handle().get_item(&id);
                        let _ = item_handle
                            .set_title(format!("Click throught : {}", &config.click_throught))
                            .unwrap();
                        let _ = window.set_ignore_cursor_events(config.click_throught);
                    }
                    None => println!("not found window"),
                },
                "config" => match app.get_window("config") {
                    Some(window) => {
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                    None => {
                        tauri::WindowBuilder::new(
                            app,
                            "config",
                            tauri::WindowUrl::App("config.html".into()),
                        )
                        .title("Config!")
                        .inner_size(800.0, 600.0)
                        .build()
                        .unwrap();
                    }
                },

                _ => {}
            },
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {}
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {}
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet, action_lists, filter_keys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
