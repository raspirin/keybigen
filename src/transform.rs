use crate::keybinding::KeyBinding;
use crate::keybinding::KeyBindingMod::{Command, Control, Shift};

pub struct TransformRules;

impl TransformRules {
    pub fn control_to_command(from: KeyBinding) -> KeyBinding {
        if let Some(mods) = from.mods {
            let new_mods = if mods.len() == 1 && mods[0] == Control {
                vec![Command]
            } else {
                mods
            };

            KeyBinding {
                mods: Some(new_mods),
                ..from
            }
        } else {
            from
        }
    }

    pub fn control_to_command_with_shift(from: KeyBinding) -> KeyBinding {
        if let Some(mods) = from.mods {
            let new_mods = if mods.len() == 2 && mods.contains(&Control) && mods.contains(&Shift) {
                vec![Command, Shift]
            } else {
                mods
            };

            KeyBinding {
                mods: Some(new_mods),
                ..from
            }
        } else {
            from
        }
    }
}
