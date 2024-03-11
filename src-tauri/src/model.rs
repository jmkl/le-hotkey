use global_hotkey::hotkey::Code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RegisteredKey {
    pub id: u32,
    pub key: String,
    pub key_code: Code,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CurrentKeyPayload {
    pub reset: bool,
    pub mod_pressed: bool,
    pub multikey: bool,
    pub key_1: String,
    pub key_2: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MacropadMode {
    Action,
    Macro,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActionData {
    fromserver: bool,
    #[serde(rename = "type")]
    action_type: String,
    data: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MacroData {
    pub key_down: bool,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KeyData {
    Macro(Vec<MacroData>),
    Action(ActionData),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacropadData {
    pub key_name: String,
    pub key_desc: String,
    pub key_mode: MacropadMode,
    pub key_multikey: bool,
    pub key_1: Option<String>,
    pub key_2: Option<String>,
    pub key_data: Option<KeyData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub click_throught: bool,
    pub always_on_top: bool,
}
