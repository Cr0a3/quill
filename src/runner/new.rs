use crate::{conf::{self, Dependencies, Package, TemplateData}, print, utils};
use std::{env, fs, io::Write, path::Path};
use PrintLib::colorize::Colorize;

pub fn new(name: &str, libary: bool, template: &str) -> std::io::Result<()>{
    let current_dir = env::current_dir()?;
    let path_str =  format!("{}/.cache/templates/{}.zip", current_dir.as_os_str().to_str().expect("couldn't get current dir"), template);
    
    let path = Path::new(&path_str);

    if !path.exists() {
        print::error("E", &format!("couldn't find template: '{}'", template));

        return Ok(());
    }

    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            print::error("", &format!("error while opening zip ({}): {}", path.display(), e.to_string()));
            return Ok(());
        },
    };

    utils::extract_zip(&".".into(), file)?;

    // rename dir to project name
    fs::rename(template, name)?;

    // rewrite config
    let template_cfg_path = format!("{}/template.toml", name);
    let template_deps = conf::parse_dependencys(&template_cfg_path);

    let data = conf::Data {
        package: Package {
            name: name.into(),
            version: "1.0.0".into(),
            author: "your_name".into(),
            description: format!("{}s epic description", name),
            lib: conf::load_tml_cfg::<TemplateData>(&template_cfg_path).lib,
        },
        dependencies: Dependencies { },
    };

    let mut toml_string = match toml::to_string(&data) {
        Ok(s) => s,
        Err(e) => {
            print::error("E", &format!("error while converting example conf to string: {}", e.to_string()));
            String::new()
        },
    };

    for (name, version) in template_deps {
        toml_string.push_str(&format!("{} = {}", name, version));
    }

    let mut file = match fs::File::create(Path::new(&format!("{}/quill.toml", name))) {
        Ok(f) => f,
        Err(e) => {
            print::error("E", &format!("error while opening conf file: {}", e));
            return Ok(());
        },
    };

    match file.write(toml_string.as_bytes()) {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("error while writing conf file: {}", e));
            return Ok(());
        },
    };

    // remove template.toml
    match fs::remove_file(Path::new(&format!("{}/template.toml", name))) {
        Ok(_) => {},
        Err(e) => {
            print::error("E", &format!("error while removing template conf file: {}", e));
            return Ok(());
        },
    };
    
    println!("  - {} {}: '{name}'", "Created".color(0, 42, 71).bold(), match libary { true => "libary", false => "package" } );

    Ok(())
}