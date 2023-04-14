use crate::cli::{Cli, CliRule};
use crate::keybinding::{KeyBinding, KeyBindings};
use crate::transform::TransformRules;
use clap::Parser;
use std::fs::File;
use std::io::Read;

mod cli;
mod keybinding;
mod transform;

fn main() {
    let cli = Cli::parse();
    let mut file = File::open(cli.path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let keybindings = toml::from_str::<KeyBindings>(s.as_str()).unwrap();
    let keybindings = keybindings.keybindings;

    let keybindings = match cli.rule {
        None => keybindings,
        Some(CliRule::Default) => keybindings
            .into_iter()
            .map(TransformRules::control_to_command)
            .collect::<Vec<_>>()
            .into_iter()
            .map(TransformRules::control_to_command_with_shift)
            .collect::<Vec<KeyBinding>>(),
    };

    println!("keybindings:");
    for i in keybindings {
        println!("{i}")
    }
}
