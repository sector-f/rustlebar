extern crate toml;

use std::io::Read;
use std::env::var;
use std::path;
use std::fs::File;

pub struct Icons {
    pub occupied_focused: String,
    pub occupied_unfocused: String,
    pub free_focused: String,
    pub free_unfocused: String,
    pub urgent_focused: String,
    pub urgent_unfocused: String,
}

fn get_config_path() -> Option<String> {
    if let Ok(value) = var("XDG_CONFIG_HOME") {
        let path = value + &path::MAIN_SEPARATOR.to_string() + "rustlebar.toml";
        if File::open(&path).is_ok() {
            Some(path)
        } else {
            None
        }
    } else if let Ok(value) = var("HOME") {
        let path = value + &path::MAIN_SEPARATOR.to_string() + ".config" + &path::MAIN_SEPARATOR.to_string() + "rustlebar.toml";
        if File::open(&path).is_ok() {
            Some(path)
        } else {
            None
        }
    } else {
        None
    }
}

fn get_value(toml: &toml::Value, default: &str, value: &str) -> String {
    if let Some(item) = toml.lookup(value) {
        match item.as_str() {
            Some(string) => string.to_owned(),
            None => default.to_owned()
        }
    } else {
        default.to_owned()
    }
}

pub fn get_icons() -> Icons {
    let mut occupied_focused = "".to_owned();
    let mut occupied_unfocused = "".to_owned();
    let mut free_focused = "".to_owned();
    let mut free_unfocused = "".to_owned();
    let mut urgent_focused = "".to_owned();
    let mut urgent_unfocused = "".to_owned();

    let mut buffer = String::new();

    if let Some(path) = get_config_path() {
        let mut file = File::open(&path).unwrap();
        let _ = file.read_to_string(&mut buffer);
        let configuration: toml::Value = buffer.parse().unwrap();

        occupied_focused = get_value(&configuration, &occupied_focused, "icons.occupied_focused");
        occupied_unfocused = get_value(&configuration, &occupied_unfocused, "icons.occupied_unfocused");
        free_focused = get_value(&configuration, &free_focused, "icons.free_focused");
        free_unfocused = get_value(&configuration, &free_unfocused, "icons.free_unfocused");
        urgent_focused = get_value(&configuration, &urgent_focused, "icons.urgent_focused");
        urgent_unfocused = get_value(&configuration, &urgent_unfocused, "icons.urgent_unfocused");

    }

    Icons {
        occupied_focused: occupied_focused,
        occupied_unfocused: occupied_unfocused,
        free_focused: free_focused,
        free_unfocused: free_unfocused,
        urgent_focused: urgent_focused,
        urgent_unfocused: urgent_unfocused,
    }
}
