use serde::Deserialize;
use std::fs;
use std::process::exit;
use toml;
use crate::print;

#[derive(Deserialize)]
pub struct Data {
    pub package: Package,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

pub fn load_tml_cfg(path: &str) -> Data {
    let contents = match fs::read_to_string(path) {
        Ok(c) => {c}
        Err(_) => {
            print::error("E005", "couldn't read config file");
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            print::error("E005", &format!("couldn't load toml file:\n{}", e));
            exit(1);
        }
    };

    data
}