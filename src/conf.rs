use serde::{Serialize, Deserialize};
use std::{fs, fs::File, io::Read, path::Path};
use std::process::exit;
use toml;
use crate::print;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub package: Package,
}

#[derive(Serialize, Deserialize)]
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

pub fn read_file(path: &str) -> std::io::Result<String> {
    let path = Path::new(&path);

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            print::error("E", &format!("error while opening conf file: {}", e));
            return Ok(String::new());
        },
    };

    let mut buf: String = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("error while reading conf file: {}", e));
            return Ok(String::new());
        },
    };

    Ok(buf)
}