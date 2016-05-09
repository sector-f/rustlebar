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

pub struct Colors {
    pub occupied_focused: String,
    pub occupied_unfocused: String,
    pub free_focused: String,
    pub free_unfocused: String,
    pub urgent_focused: String,
    pub urgent_unfocused: String,
}

pub struct LemonbarOptions {
    pub width: String,
    pub height: String,
    pub x: String,
    pub y: String,
    pub text_font: String,
    pub icon_font: String,
    pub title_length: String,
    pub background_color: String,
    pub clickable_areas: String,
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

pub fn get_colors() -> Colors {
    let mut buffer = String::new();

    if let Some(mut f) = get_config_path().and_then(|p| File::open(p).ok()) {
        f.read_to_string(&mut buffer).expect("Can't read configuration file");
    }

    let configuration: toml::Value = buffer.parse().unwrap_or(toml::Value::Array(Vec::new()));

    Colors {
        occupied_focused: get_value(&configuration, "#FFF6F9FF", "colors.occupied_focused"),
        occupied_unfocused: get_value(&configuration, "#FFA3A6AB", "colors.occupied_unfocused"),
        free_focused: get_value(&configuration, "#FFF6F9FF", "colors.free_focused"),
        free_unfocused: get_value(&configuration, "#FF6F7277", "colors.free_unfocused"),
        urgent_focused: get_value(&configuration, "#FF916255", "colors.urgent_focused"),
        urgent_unfocused: get_value(&configuration, "#FF543B3B", "colors.urgent_unfocused"),
    }
}

pub fn get_lemonbar_options() -> LemonbarOptions {
    let mut buffer = String::new();

    if let Some(mut f) = get_config_path().and_then(|p| File::open(p).ok()) {
        f.read_to_string(&mut buffer).expect("Can't read configuration file");
    }

    let configuration: toml::Value = buffer.parse().unwrap_or(toml::Value::Array(Vec::new()));

    LemonbarOptions {
        width: get_value(&configuration, "1920", "lemonbar.width"),
        height: get_value(&configuration, "30", "lemonbar.height"),
        x: get_value(&configuration, "0", "lemonbar.x"),
        y: get_value(&configuration, "0", "lemonbar.y"),
        text_font: get_value(&configuration, "DejaVu Sans Mono-13", "lemonbar.text_font"),
        icon_font: get_value(&configuration, "FontAwesome-15", "lemonbar.icon_font"),
        title_length: get_value(&configuration, "100", "lemonbar.title_length"),
        background_color: get_value(&configuration, "#141314", "lemonbar.background"),
        clickable_areas: get_value(&configuration, "30", "lemonbar.clickable"),
    }
}
