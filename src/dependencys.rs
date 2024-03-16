use std::{env, fs::{self, OpenOptions}, io::Write, path::{Path, PathBuf}, process::Command};
use PrintLib::colorize::Colorize;
use crate::{api, conf::{self, Data}, consts, print, utils};

pub fn get_exe_path() -> PathBuf {
    let bin_path = match env::current_exe() {
        Ok(p) => { p }
        Err(e) => {
            print::error("E", &format!("could not get current path: {}", e));
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

pub fn is_installed(name: &String, version: &String) -> bool {
    let binary_path = get_bin_path();

    if !setuped() {
        setup_dirs();
    }

    let mut fmt_path: String;

    if version == &String::new() { // no version
        println!("any version");

        fmt_path = format!("{}/.cache/lib_{name}_", binary_path); 

        return true;
    } else {
        fmt_path = format!("{}/.cache/lib_{name}_{version}", binary_path); 
    }
    let path = Path::new(&fmt_path);

        if path.exists() {
            return true
        } else {
            return false;
        }
}

pub fn compile(name: &String, version: &String, target: &String) -> bool {
    let installed = is_installed(&name, &version);
    match installed {
        true => {},
        false => {
            print::error("E", &format!("libary '{}' isn't installed", name.clone()));
        }
    };

    let lib_path = format!("{}/.cache/lib_{name}_{version}/", get_bin_path());

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
    if is_installed(&name, &version) {
        return false;
    }

    let api = api::Api::new(consts::DOMAIN);
    let path = format!("{}/.cache/{name}.zip", get_bin_path());

    if !match api.download(&name, &version, &path).await {
        Ok(b) => b,
        Err(e) => {
            print::error("E", &format!("error while getting download link: {}", e));
            return false;
        },
    } { return false; }

    if !install_lib_from_zip(&path) { return false; }

    println!(" {} {name} v{version}", "Downloaded".bold().color(0, 42, 71));
    
    true
}

pub fn install_lib_from_zip(path: &String) -> bool {
    if !setup_dirs() { return false };

    let file = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(e) => {
            print::error("E", &format!("error opening/creating extracted zip file: {}", e));
            return false;
        },
    };

    let zip_name = Path::new(path).file_stem().expect("error while unwraping file name").to_str().expect("error while chainging file name to &str");
    let mut extract_path = format!("{}/.cache/", get_bin_path());

    match utils::extract_zip(&extract_path, file) {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("error while extracting zip: {}", e));
            return false;
        },
    };

    extract_path = format!("{extract_path}/{zip_name}");

    let package = conf::load_tml_cfg::<Data>(&format!("{extract_path}/quill.toml")).package;
    let lib_name = package.name;
    let lib_version = package.version;

    match fs::rename(&extract_path, extract_path.replace(zip_name, &format!("lib_{lib_name}_{lib_version}"))) {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("error while renaming installation: {}", e));
            return false;
        },
    };

    true
}

pub fn setup_dirs() -> bool {
    if setuped() { return true; }

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
    let target_path = format!("target/{target}/{libary_name}.{}", consts::LIBARY_EXT);
    let libary_path = format!("{}/.cache/lib_{libary_name}/target/{target}/{libary_name}.{}", get_bin_path(), consts::LIBARY_EXT);

    if ! Path::new(&libary_path).exists() {
        print::error("E", &format!("libarys '{libary_name}' build dosn't exists"));
        return false;
    }

    match fs::copy(libary_path, target_path) {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("error while copying libary {}: {}", consts::LIBARY_EXT, e));
        },
    };

    true
}

pub fn copy_lib_include_to_current_package(name: &String, version: &String) -> bool {
    false
}

pub fn add_lib_to_current_conf(name: &String, version: &String) -> bool {
    if conf::parse_dependencys("./quill.toml").contains_key(name) { // dependency allready added to conf
        return false;
    }

    let mut file  = match OpenOptions::new().append(true).open("./quill.toml") {
        Ok(f) => f,
        Err(e) => {
            print::error("E", &format!("error while opening conf file: {}", e));
            return false
        },
    };

    match write!(file, "\"{name}\" = \"{version}\"") {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("error while writing to conf file: {}", e));
            return false
        },
    };

    true
}