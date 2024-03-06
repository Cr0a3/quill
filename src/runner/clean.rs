use PrintLib::colorize::Colorize;
use std::{fs, path::Path};
use crate::print;

pub fn clean() {
    let dir = Path::new("./target");

    if dir.exists() {
        match fs::remove_dir_all(dir) {
            Ok(_) => {
                print!(" {} builds", "Cleaned".bold().color(0, 42, 71));
            },
            Err(e) => {
                print::error("E", &format!("error while removing target directory: {}", e.to_string()));
            },
        };
    }
}