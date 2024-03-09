use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::process::exit;
use struct_iterable::Iterable;
use toml;
use crate::print;

#[derive(Serialize, Deserialize)]
pub struct TemplateData {
    
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub package: Package,
    pub dependencies: Dependencies,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

#[derive(Iterable, Serialize, Deserialize)]
pub struct Dependencies {
}

pub fn get_value(path: &str) -> toml::Value {
    let contents = match fs::read_to_string(path) {
        Ok(c) => {c}
        Err(e) => {
            print::error("E", &format!("couldn't read config file: {}", e.to_string()));
            exit(1);
        }
    };

    let data: toml::Value = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            print::error("E", &format!("couldn't load toml file:\n{}", e));
            exit(1);
        }
    };

    data
}

pub fn load_tml_cfg(path: &str) -> Data {
    let contents = match fs::read_to_string(path) {
        Ok(c) => {c}
        Err(e) => {
            print::error("E", &format!("couldn't read config file: {}", e.to_string()));
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            print::error("E", &format!("couldn't load toml file:\n{}", e));
            exit(1);
        }
    };

    data
}

pub fn parse_dependencys(path: &str) -> HashMap<String, String> {
    let value = get_value(path);

    let keys_table: &toml::map::Map<String, toml::Value> = value.get("dependencies").unwrap().as_table().unwrap();
    let mut keys = std::collections::HashMap::new();

    for (key, value) in keys_table.iter() {
        keys.insert(key.clone(), value.as_str().unwrap().to_string());
    }

    keys
}