extern crate toml;

use std::io::Read;
use std::env;
use std::path::PathBuf;
use std::fs::File;

pub struct Icons {
    pub occupied_focused: String,
    pub occupied_unfocused: String,
    pub free_focused: String,
    pub free_unfocused: String,
    pub urgent_focused: String,
    pub urgent_unfocused: String,
}

fn if_readable(path: PathBuf) -> Option<PathBuf> { if path.exists() { Some(path) } else { None } }

fn get_config_path() -> Option<PathBuf> {
    let xdg_path = env::var("XDG_CONFIG_HOME").ok()
        .map(|v| PathBuf::from(v).join("rustlebar.toml"))
        .and_then(if_readable);

    let dot_home = env::var("HOME").ok()
        .map(|v| PathBuf::from(v).join(".config").join("rustlebar.toml"))
        .and_then(if_readable);

    xdg_path.or(dot_home)
}

fn get_value(toml: &toml::Value, default: &str, value: &str) -> String {
    toml.lookup(value)
        .and_then(toml::Value::as_str).map(str::to_owned)
        .unwrap_or_else(|| default.to_owned())
}

pub fn get_icons() -> Icons {
    let mut buffer = String::new();

    if let Some(mut f) = get_config_path().and_then(|p| File::open(p).ok()) {
        f.read_to_string(&mut buffer).expect("Can't read configuration file");
    }

    let configuration: toml::Value = buffer.parse().unwrap_or(toml::Value::Array(Vec::new()));

    Icons {
        occupied_focused: get_value(&configuration, "", "icons.occupied_focused"),
        occupied_unfocused: get_value(&configuration, "", "icons.occupied_unfocused"),
        free_focused: get_value(&configuration, "", "icons.free_focused"),
        free_unfocused: get_value(&configuration, "", "icons.free_unfocused"),
        urgent_focused: get_value(&configuration, "", "icons.urgent_focused"),
        urgent_unfocused: get_value(&configuration, "", "icons.urgent_unfocused"),
    }
}
