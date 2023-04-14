use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum KeyBindingMod {
    Command,
    Control,
    Alt,
    Option,
    Super,
    Shift,
}

impl Display for KeyBindingMod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            KeyBindingMod::Command => "Command",
            KeyBindingMod::Control => "Control",
            KeyBindingMod::Alt => "Alt",
            KeyBindingMod::Option => "Option",
            KeyBindingMod::Super => "Super",
            KeyBindingMod::Shift => "Shift",
        };
        write!(f, "{}", w)
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: String,
    pub mods: Option<Vec<KeyBindingMod>>,
    pub mode: Option<String>,
    pub chars: Option<String>,
    pub action: Option<String>,
}

impl KeyBinding {
    fn mods_string(&self) -> Option<String> {
        if let Some(mods) = &self.mods {
            let mut s: Vec<String> = Vec::new();
            for i in 0..mods.len() {
                s.push(mods[i].to_string());
                if i != mods.len() - 1 {
                    s.push("|".to_string());
                }
            }
            let mut w = String::new();
            for i in s {
                w = format!("{}{}", w, i);
            }
            Some(w)
        } else {
            None
        }
    }
}

impl Display for KeyBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = "- {".to_string();
        let mut w = format!("{} key: {},", w, self.key);
        if let Some(mods) = self.mods_string() {
            w = format!("{} mods: {}", w, mods);
        }
        if let Some(mode) = &self.mode {
            w = format!("{}, mode: {}", w, mode);
        }
        if let Some(chars) = &self.chars {
            w = format!("{}, chars: \"{}\"", w, chars);
        }
        if let Some(action) = &self.action {
            w = format!("{}, action: {}", w, action);
        }
        w = format!("{} }}", w);
        write!(f, "{}", w)
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyBindings {
    pub keybindings: Vec<KeyBinding>,
}
