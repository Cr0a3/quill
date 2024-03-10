use crate::{conf::{self, Data}, print};
use std::{path::Path, process::Command};
use PrintLib::colorize::Colorize;
use crate::runner::build::build;

pub fn run(target: &str, noout: bool) -> Option<bool> {
    // read toml
    let name = conf::load_tml_cfg::<Data>("cpack.toml").package.name;

    // filter out compile errors
    let sucess = match build(target, noout) {
        Ok(b) => b,
        Err(e) => {
            print::error("E", &format!("error while compiling: {}", e.to_string()));
            return Some(false);
        },
    };

    if sucess == false {
        return Some(false);
    }
    
    // now there are no compile errors
    let fmt_path = format!("target/{target}/{}.exe", name);
    let bin = Path::new( &fmt_path );

    if !bin.exists() {
        print::error("E", "binary doesn't exists")
    }

    // run
    let mut cmd = Command::new(bin);
    let status = cmd.status();

    match status {
        Ok(s) => {
            if s.success() && !noout {
                println!("\n  - {} {}", "Program exited sucessfull with code".bold().green(), s.code()?);
            } else {
                if !noout { println!("\n  - {} {}", "Program didn't exit sucessfull with code".bold().red(), s.code()?); }
            }
            Some(true)
        },
        Err(e) => {
            print::error("E", &format!("error while executing command: {e}"));
            Some(false)
        },
    }
}