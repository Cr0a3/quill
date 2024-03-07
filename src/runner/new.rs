use crate::print;
use std::{env, fs, path::Path};
use PrintLib::colorize::Colorize;

pub fn new(name: &str, libary: bool, template: &str) -> std::io::Result<()>{
    println!("name:     {}", name);
    println!("libary:   {}", libary);
    println!("template: {}", template);

    let current_dir = env::current_dir()?;
    let path_str =  format!("{}/templates/{}.zip", current_dir.as_os_str().to_str().expect("couldn't get current dir"), template);
    
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

    let mut zip = zip::ZipArchive::new(file)?;

    // extracting zip
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
    }

    /*// writing files
    let data = Data {
        package: Package {
            name: name.into(),
            version: "".into(),
            author: "".into(),
            description: "".into(),
        }
    };

    let toml = match toml::to_string(
        &data ) {
        Ok(d) => { d },
        Err(e) => {
            print::error("E", &format!("couldn't serialize example data: {}", e.to_string()) );
            return;
        }
    };

    match conf.write(toml.as_bytes()) {
        Ok(_) => {},

        Err(e) => {
            print::error("E", &format!("error while writing cpack.toml: {}", e.to_string()));
            return;
        }
    }*/

    println!("  - {} {}: '{name}'", "Created".color(0, 42, 71).bold(), match libary { true => "libary", false => "package" } );

    Ok(())
}