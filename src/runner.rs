pub mod build;
pub mod clean;
pub mod new;
pub mod run;
use std::fs;

use crate::{api::Api, conf::{self, Data}, consts, dependencys::*, print, utils};
use PrintLib::colorize::Colorize;

pub async fn publish() -> bool {
    // read toml
    let package = conf::load_tml_cfg::<Data>("quill.toml").package;
    let name = package.name;
    let version = package.version;
    let lib = package.lib.unwrap_or(false);
    
    println!("{}",  "Publishing ".green() + &name.green());

    if !lib {
        println!("{}", lib);

        print::error("E", "current package needs to be a libary");
        return false; 
    }

    // check if it compiles correctly
    if !build::build(&"release".to_string(), false).await.expect("error while building") { return false; }

    let _ = fs::remove_dir_all("target");

    // package to zip
    let outpath = format!("{name}_{version}.zip");

    match utils::zip(&outpath, &".".into()) {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("couldn't zip current dir: {}", e));
        },
    };

    // upload to site
    let api = Api::new(consts::DOMAIN);
    match api.upload(&outpath).await {
        Ok(b) => {
            if !b { return false; }
        },
        Err(e) => {
            print::error("E", &format!("couldn't upload: {}", e));
        },
    };

    print!("after publishing");

    false
}

pub async fn add(name: String) -> bool {

    let version: String;

    if name.contains("=") {
        version = name.split_once("=").expect("error while parsing version (runner.rs/26)").0.into();
    } else {
        let api = Api::new(consts::DOMAIN);

        version = match api.latest(&name).await {
            Ok(s) => { s },
            Err(e) => {
                print::error("E", &format!("cann't get latest version of {name}: {}", e));
                return false;
            },
        };
    }

    if name.contains(".zip") {
        if !install_lib_from_zip(&name) { return false; };
    } else {
        if !is_installed(&name, &version) {
            if !download(name.clone(), version.clone()).await { return false; };
        }
    }

    if !add_lib_to_current_conf(&name, &version) { return false };

    copy_lib_include_to_current_package(&name,  &version)
}