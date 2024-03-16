pub mod build;
pub mod clean;
pub mod new;
pub mod run;
use crate::{api::Api, conf::{self, Data}, consts, dependencys::*, print, utils};
use semver::VersionReq;

pub async fn publish() -> bool {
    // read toml
    let package = conf::load_tml_cfg::<Data>("quill.toml").package;
    let name = package.name;
    let version = package.version;

    if package.lib.unwrap_or(false) { return false; }

    // check if it compiles correctly
    if !compile(&name, &version, &"release".to_string()) { return false; }

    // package to zip
    let outpath = format!("target/{name}_{version}.zip");

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

    false
}

pub async fn add(name: String) -> bool {

    if name.contains("=") {
        let version_str = name.split_once("=").expect("error while parsing version (runner.rs/26)").0;

    }

    if name.contains(".zip") {
        if !install_lib_from_zip(&name) { return false; };
    } else {
        if !is_installed(&name, &String::new()) {
            if !download(name.clone(), "latest".into()).await { return false; };
        }
    }

    if !add_lib_to_current_conf(&name, &String::new()) { return false };

    copy_lib_include_to_current_package(&name,  &String::new())
}