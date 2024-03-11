use crate::{conf::{self, parse_dependencys, Data}, consts, dependencys::*, print};
use std::{fs, process::Command};
use PrintLib::colorize::Colorize;

pub async fn build(target: &str, noout: bool) -> Result<bool, std::io::Error> {
    let data = conf::load_tml_cfg::<Data>("cpack.toml");
    if !noout { println!("{} | {}", 
    "Building ".green() + &data.package.name.green(), 
    "Target: ".color(0, 42, 71) + &target.color(0, 42, 71)); }

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

    //print dependencies
    let deps = parse_dependencys("cpack.toml");
    for (name, version) in &deps {
        let installed = is_installed(&name);
        if installed {
            if !compile(&name, &target.into()) {
                return Ok(false);
            }
        } else {
            if download(name.clone(), version.into()).await {
                if !compile(&name, &target.into()) {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        // copy libary dll to current folder
        let _ = copy_libary_build_to_current_target(name.into(), target.into());
    }

    // compile every file
    for file in src_dir {
        let file = file?;
        let path = file.path();
        let name = path.display().to_string();
        let file_name = path.file_name().unwrap().to_str().unwrap();


        let mut cmd = Command::new("g++");
        cmd.current_dir(".");
        
        cmd.arg("-c");

        cmd.arg(format!("{}", name));
        cmd.arg("-o");
        cmd.arg(
            format!("target/{}/objs/{}.o", target, file_name)
        );
        cmd.arg("-Iinclude");
        cmd.arg("-Isrc");

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
    let lib = conf::load_tml_cfg::<Data>("cpack.toml").package.lib.unwrap_or(false);

    let ext = match lib {false => consts::BINARY_EXT, true => consts::LIBARY_EXT };
    let prog = match lib {false => "g++", true => "ld" };

    if sucess {
        let bins = fs::read_dir(format!("target/{target}/objs/"))?;

        let mut cmd = Command::new(prog);
        cmd.current_dir(".");

        for file in bins {
            let path = file?.path().display().to_string();
            cmd.arg(path);
        }

        cmd.arg(format!("-Ltarget/{target}"));

        for (name, _) in deps {
            cmd.arg(
                format!("{}{name}{}", consts::LIBARY_LINK_LD_OPT, consts::LIBARY_LINK_LD_OPTI)
            );
        }

        cmd.arg("-o");
        cmd.arg(
            format!("target/{target}/{}.{ext}", data.package.name)
        );

        if lib {
            cmd.arg(consts::LIBARY_LD_FLAG);
        } else {
            cmd.arg("-lstdc++");
        }

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
        println!("  {} error", "Build".bold().red());
    }

    Ok(sucess)
}