use std::time::Instant;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use tauri::Manager;

use crate::{
    config::read_macropad_config,
    model::{CurrentKeyPayload, RegisteredKey},
    APP_HANDLE,
};
pub struct MacropPad {
    pub registered_keys: Vec<RegisteredKey>,
    pub mod_pressed: bool,
    pub multikey: bool,
    last_pressed: Instant,
    reset: bool,
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
            multikey: false,
            reset: false,
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
        self.multikey = false;
        self.long_pressed = false;
    }

    fn broadcast_keys(&self) {
        //println!("broadcast: {:?}{:?}", self.key_1, self.key_2);
        let ckey_1: String = match self.key_1 {
            Some(key1) => key1.to_string(),
            None => String::from(""),
        };
        let ckey_2: String = match self.key_2 {
            Some(key2) => key2.to_string(),
            None => String::from(""),
        };
        let current_keys = CurrentKeyPayload {
            reset: self.reset,
            mod_pressed: self.mod_pressed,
            multikey: self.multikey,
            key_1: ckey_1,
            key_2: ckey_2,
        };

        let _ = APP_HANDLE
            .get()
            .unwrap()
            .emit_all("modkey_event", &current_keys);
    }

    pub fn process_key(&mut self, event: &GlobalHotKeyEvent) {
        if let Some(key) = self.registered_keys.iter().find(|&k| k.id == event.id) {
            self.reset = false;

            if event.state == HotKeyState::Pressed {
                if key.key_code == Code::Insert {
                    self.last_pressed = Instant::now();
                    let _ = APP_HANDLE.get().unwrap().emit_all("mod_pressed", true);
                }
            }

            if event.state == HotKeyState::Released {
                if key.key_code == Code::Insert {
                    let milisecond = (Instant::now() - self.last_pressed).as_millis();
                    self.long_pressed = milisecond > 500;
                    println!("milisecond {:?}", milisecond);
                    let _ = APP_HANDLE.get().unwrap().emit_all("mod_pressed", false);

                    if self.long_pressed {
                        self.reset = true;
                        self.broadcast_keys();
                        self.reset_keys();
                        return;
                    }
                    self.reset_keys();

                    self.multikey = true;
                    self.mod_pressed = true;
                    self.broadcast_keys();

                    return;
                }
                if !self.mod_pressed {
                    if key.key_code == Code::Insert {
                        self.mod_pressed = true;
                        return;
                    }
                    self.key_1 = Some(key.key_code);

                    self.execute_single_key();
                    self.broadcast_keys();
                    self.reset_keys();
                    return;
                }
                if self.key_1 == Option::None {
                    self.key_1 = Some(key.key_code);
                    self.multikey = true;
                    self.broadcast_keys();
                    return;
                }
                if self.key_2 == Option::None {
                    self.key_2 = Some(key.key_code);
                    self.multikey = true;
                    self.reset = true
                }
                self.broadcast_keys();
                self.execute_multiple_keys();
                self.reset_keys();
            }
        }
    }

    pub fn execute_single_key(&self) {
        let single_key = match self.key_1 {
            Some(sk) => sk.to_string(),
            None => "".to_string(),
        };
        match read_macropad_config() {
            Ok(mpad) => {
                if let Some(obj) = mpad.iter().find(|&o| {
                    o.key_1.as_deref().map_or(false, |s| s == single_key)
                        && o.key_2.as_deref().map_or(false, |s| s == "")
                }) {
                    println!("{:?}", obj.key_data);
                }
            }
            Err(_) => {}
        }
        //println!("singlekey: {:?}", key);
    }
    pub fn execute_multiple_keys(&self) {
        if !self.multikey {
            return;
        }
        let key_1 = match self.key_1 {
            Some(sk) => sk.to_string(),
            None => "".to_string(),
        };
        let key_2 = match self.key_2 {
            Some(sk) => sk.to_string(),
            None => "".to_string(),
        };
        match read_macropad_config() {
            Ok(mpad) => {
                if let Some(obj) = mpad.iter().find(|&o| {
                    o.key_1.as_deref().map_or(false, |s| s == key_1)
                        && o.key_2.as_deref().map_or(false, |s| s == key_2)
                }) {
                    println!("{:?}", obj.key_data);
                }
            }
            Err(_) => {}
        }
    }
}
