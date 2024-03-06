use PrintLib::colorize::Colorize;
use std::{fs, path::Path};
use crate::conf::{self};

pub fn clean(target: &str) {
    let data = conf::load_tml_cfg("cpack.toml");
    println!("{} | {}", 
    "Cleaning ".green() + &data.package.name.green(), 
    "Target: ".blue() + &target.color(0, 42, 71));

    let dir = Path::new("target");

    if dir.exists() {
        fs::remove_dir(dir);
    }
}