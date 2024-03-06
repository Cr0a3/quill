use crate::{conf::{self}, print};
use std::{fs, process::Command};
use PrintLib::colorize::Colorize;

pub fn build(target: &str) -> Result<bool, std::io::Error> {
    let data = conf::load_tml_cfg("cpack.toml");
    println!("{} | {}", 
    "Building ".green() + &data.package.name.green(), 
    "Target: ".color(0, 42, 71) + &target.color(0, 42, 71));

    let mut args: Vec<String> = vec!["-Iinclude".into(), "-Isrc".into(), "-c".into()];

    match target {
        "debug" => {
            args.push("-g".into());
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

            return Ok(false);
        }
    }

    let mut sucess: bool = true;

    let src_dir = fs::read_dir("./src")?;

    //mkdir
    if  let Err(err) = fs::create_dir_all(format!("target/{}/objs", target)) {
        print::error(
            "E008",
             &format!("Error while creating target folder {}", err.to_string())
            );
        return Ok(false);
    }

    // compile every file
    for file in src_dir {
        let file = file?;
        let path = file.path();
        let name = path.display().to_string();
        let file_name = path.file_name().unwrap().to_str().unwrap();


        let mut cmd = Command::new("g++");
        cmd.current_dir(".");
        
        cmd.args(args.clone());

        cmd.arg(format!("{}", name));
        cmd.arg("-o");
        cmd.arg(
            format!("target/{}/objs/{}.o", target, file_name)
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
                return Ok(false);
            },
        }
    }

    // link together
    if sucess {
        let bins = fs::read_dir(format!("target/{target}/objs/"))?;

        let mut cmd = Command::new("g++");
        cmd.current_dir(".");

        for file in bins {
            let path = file?.path().display().to_string();
            cmd.arg(path);
        }

       cmd.arg("-o");
       cmd.arg(
        format!("target/{target}/{}.exe", data.package.name)
       );

        let status = cmd.status();

        match status {
            Ok(stat) => {
                if !stat.success() {
                    sucess = false;
                }
            },
            Err(e) => {
                print::error("E007", format!("could not start the linker: {}", e).as_str());
                return Ok(false);
            },
        }
    } else {
        print!("{}", "Build error".bold().red());
    }

    Ok(sucess)
}