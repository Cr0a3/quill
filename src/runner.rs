use crate::{print, conf};
use std::{fs, process::Command};
use PrintLib::colorize::Colorize;

pub fn build(target: &str) -> bool {
    let data = conf::load_tml_cfg("cpack.toml");
    println!("{} {} | {}", 
    "Building ".green() + &data.package.name.green(), 
    "v".green() + &data.package.version.green(), 
    "Target: ".color(0, 42, 71) + &target.color(0, 42, 71));

    let mut args: Vec<String> = vec!["-Iinclude".into(), "-Isrc".into(), "-c".into()];

    match target {
        "debug" => {
            args.push("-d".into());
        }
        "release" => {
            args.push("-O3".into());
        }
        _=> {
            let fab = PrintLib::error::ErrorFactory::new("E006".into(), format!("unknown build target '{}'", target));
            fab.print();

            println!("  {}", "Known targets:".gray());
            println!("    - {}", "debug".color(0, 42, 71).bold());
            println!("    - {}", "release".color(0, 42, 71).bold());

            return false;
        }
    }

    let mut sucess: bool = false;

    let src_dir = fs::read_dir("./src").unwrap();

    //mkdir
    if  let Err(err) = fs::create_dir_all(format!("target/{}/objs", target)) {
        print::error(
            "E008",
             &format!("Error while creating target folder {}", err.to_string())
            );
        return false;
    }

    for file in src_dir {
        let path = file.unwrap().path();
        let name = path.display().to_string();

        let mut cmd = Command::new("g++");
        
        for arg in args.iter() {
            cmd.arg(arg);
        }

        cmd.arg(format!("{}", name));
        cmd.arg("-o");
        cmd.arg(
            format!("target/{}/objs/{}.o", target, name)
        );

        let status = cmd.status();

        match status {
            Ok(stat) => {
                if !stat.success() {
                    sucess = false;
                }
            },
            Err(e) => {
                print::error("E007", format!("could not start the compiler: {}", e).as_str());
                return false;
            },
        }
    }

    sucess
}

pub fn clean(target: &str) {
    let data = conf::load_tml_cfg("cpack.toml");
    println!("{} | {}", 
    "Cleaning ".green() + &data.package.name.green(), 
    "Target: ".blue() + &target.blue());
}

pub fn new(name: &str) {

}

pub fn run(target: &str) {
    build(target);
}

pub fn publish() {
    /*let call = api::ApiCall::new(
        "publish".to_string(), "thispackage".as_bytes()
    ).call();

    match call {
        _ => { // worked
            PrintLib::Logger::Logger::new().info(call.unwrap().to_string());
        }
        Err(e) => {
            print::error("E004", "error while calling api", &e.to_string())
        }
    }*/
}

pub fn add(name: String) {

}