use std::time::Instant;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use tauri::Manager;

use crate::{
    model::{CurrentKeyPayload, RegisteredKey},
    APP_HANDLE,
};
pub struct MacropPad {
    pub registered_keys: Vec<RegisteredKey>,
    pub mod_pressed: bool,
    last_pressed: Instant,
    pub long_pressed: bool,
    pub key_1: Option<Code>,
    pub key_2: Option<Code>,
}
impl MacropPad {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            registered_keys: Vec::new(),
            mod_pressed: false,
            long_pressed: false,
            last_pressed: Instant::now(),
            key_1: Option::None,
            key_2: Option::None,
        })
    }
    pub fn register(&mut self) -> GlobalHotKeyManager {
        let hotkey_manager = GlobalHotKeyManager::new().unwrap();
        let hotkey_rows = vec![
            Code::F13,
            Code::F14,
            Code::F15,
            Code::F16,
            Code::F17,
            Code::F18,
            Code::F19,
            Code::F20,
            Code::F21,
            Code::F22,
            Code::F23,
            Code::Insert,
        ];
        for hotkey in hotkey_rows {
            let hkey = HotKey::new(Some(Modifiers::SHIFT), hotkey);
            let current_key = RegisteredKey {
                id: hkey.id(),
                key: hotkey.to_string(),
                key_code: hotkey,
            };
            self.registered_keys.push(current_key);
            hotkey_manager.register(hkey).unwrap();
        }
        hotkey_manager
    }

    fn reset_keys(&mut self) {
        self.key_1 = Option::None;
        self.key_2 = Option::None;
        self.mod_pressed = false;
        self.long_pressed = false;
    }

    fn broadcast_keys(&self, current_keys: &CurrentKeyPayload) {
        let _ = APP_HANDLE
            .get()
            .unwrap()
            .emit_all("modkey_event", &current_keys);
    }

    pub fn process_key(&mut self, event: &GlobalHotKeyEvent) {
        if let Some(key) = self.registered_keys.iter().find(|&k| k.id == event.id) {
            let mut current_keys = CurrentKeyPayload {
                reset: false,
                mod_pressed: false,
                key_1: "".into(),
                key_2: "".into(),
            };
            if event.state == HotKeyState::Pressed {
                if key.key_code == Code::Insert {
                    self.last_pressed = Instant::now();
                    let _ = APP_HANDLE.get().unwrap().emit_all("mod_pressed", true);
                }
            }

            if event.state == HotKeyState::Released {
                if key.key_code == Code::Insert {
                    self.long_pressed = (Instant::now() - self.last_pressed).as_millis() > 500;
                    let _ = APP_HANDLE.get().unwrap().emit_all("mod_pressed", false);

                    if self.long_pressed {
                        current_keys.reset = true;
                        self.broadcast_keys(&current_keys);
                        self.reset_keys();
                        return;
                    }
                    self.reset_keys();
                    self.mod_pressed = true;
                    current_keys.mod_pressed = self.mod_pressed;
                    self.broadcast_keys(&current_keys);

                    return;
                }
                if !self.mod_pressed {
                    if key.key_code == Code::Insert {
                        self.mod_pressed = true;

                        return;
                    }

                    self.execute_single_key(&key);
                    current_keys.key_1 = key.key_code.to_string();
                    current_keys.mod_pressed = self.mod_pressed;

                    self.broadcast_keys(&current_keys);

                    self.reset_keys();
                    return;
                }
                if self.key_1 == Option::None {
                    self.key_1 = Some(key.key_code);
                    if let Some(k) = self.key_1 {
                        current_keys.key_1 = k.to_string();
                    }

                    current_keys.mod_pressed = self.mod_pressed;

                    self.broadcast_keys(&current_keys);

                    return;
                }
                if self.key_2 == Option::None {
                    self.key_2 = Some(key.key_code);
                    if let Some(k) = self.key_1 {
                        current_keys.key_1 = k.to_string();
                    }
                    if let Some(k) = self.key_2 {
                        current_keys.key_2 = k.to_string();
                    }
                    current_keys.mod_pressed = false;
                    current_keys.reset = true;
                }
                self.broadcast_keys(&current_keys);
                self.execute_multiple_keys();
                self.reset_keys();
            }
        }
    }

    pub fn execute_single_key(&self, key: &RegisteredKey) {
        println!("singlekey: {:?}", key);
    }
    pub fn execute_multiple_keys(&self) {
        println!("{:?}{:?}", self.key_1, self.key_2)
    }
}
