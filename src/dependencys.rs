use std::{env, fs, path::{Path, PathBuf}, process::Command};
use PrintLib::colorize::Colorize;
use crate::{api, consts, print};

pub fn get_exe_path() -> PathBuf {
    let bin_path = match env::current_exe() {
        Ok(p) => { p }
        Err(e) => {
            print::error("Ee006", &format!("could not get current path: {}", e));
            return PathBuf::new();
        }
    };

    bin_path
}

pub fn get_bin_path() -> String {
    let exe_path = get_exe_path();
    let parent = exe_path.parent().unwrap();
    format!("{}", parent.display())
}

pub fn is_installed(name: &String) -> bool {
    let binary_path = get_bin_path();

    if !setuped() {
        setup_dirs();
    }

    let fmt_path = &format!("{}/.cache/lib_{}", binary_path, name);
    let path = Path::new(fmt_path);

    if path.exists() {
        return true
    } else {
        return false;
    }
}

pub fn compile(name: &String, target: &String) -> bool {
    let installed = is_installed(&name);
    match installed {
        true => {},
        false => {
            print::error("E", &format!("libary '{}' isn't installed", name.clone()));
        }
    };

    let lib_path = format!("{}/.cache/lib_{}/", get_bin_path(), name);

    let mut cmd = Command::new(get_exe_path());
    cmd.current_dir(lib_path);

    cmd.arg("--noout");

    cmd.arg("build");
    cmd.arg(target);

    let status = cmd.status();

    match status {
        Ok(s) =>  {
            if s.success() {
                println!("  {} {}", "Compiled".bold().color(0, 42, 71), name);
                return true;
            } else {
                return false;
            }
        },
        Err(e) =>  {
            print::error("E", &format!("error while starting compiling libary '{name}': {}", e.to_string()));
            return false;
        },
    };
}

pub async fn download(name: String, version: String) -> bool {
    if is_installed(&name) {
        return false;
    }

    let api = api::Api::new(consts::DOMAIN);
    let download_link = api.get_download_link(&name, &version).await;

    println!("Download link: {}", download_link);

    println!(" {} {name} v{version}", "Downloaded".bold().color(0, 42, 71));
    
    true
}

pub fn setup_dirs() -> bool {
    let path = format!("{}/.cache/", get_bin_path());
    println!("path: {}", path);

    match fs::create_dir(path) {
        Ok(_) => true,
        Err(e) => {
            print::error("E", &format!("error while creating .cache: {}", e));
            false
        },
    }
}

pub fn setuped() -> bool {
    Path::new(&format!("{}/.cache/", get_bin_path())).exists()
}

pub fn copy_libary_build_to_current_target(libary_name: String, target: String) -> bool {
    let target_path = format!("target/{target}/{libary_name}.dll");
    let libary_path = format!("{}/.cache/lib_{libary_name}/target/{target}/{libary_name}.dll", get_bin_path());

    if ! Path::new(&libary_path).exists() {
        print::error("E", &format!("libarys '{libary_name}' build dosn't exists"));
        return false;
    }

    match fs::copy(libary_path, target_path) {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("error while copying libary dll: {}", e));
        },
    };

    true
}